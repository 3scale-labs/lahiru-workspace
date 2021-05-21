use chrono::offset::Utc;
use chrono::DateTime;
use log::{info, trace};
use proxy_wasm::{
    traits::{Context, HttpContext, RootContext},
    types::{Action, ContextType, LogLevel},
};
use serde::Deserialize;
use serde::Serialize;
use serde_aux::prelude::*;
use std::time::Duration;

const CACHE_KEY: &str = "ratelimit";
const INITIALISATION_TICK: Duration = Duration::from_secs(10);

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
struct FilterConfig {
    /// Indicates the total number of requests allowed in a certain time window.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    ratelimit_limit: u16,

    /// Indicates the remaining number of requests in a certain time window.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    ratelimit_remaining: u16,

    /// Indicates the time window for ratelimiting connections.
    #[serde(with = "serde_humanize_rs")]
    ratelimit_reset: Duration,
}

// Default configuration for the ratelimit filter.
impl Default for FilterConfig {
    fn default() -> Self {
        FilterConfig {
            ratelimit_limit: 10,
            ratelimit_remaining: 10,
            ratelimit_reset: Duration::from_secs(30),
        }
    }
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|context_id| -> Box<dyn RootContext> {
        Box::new(RateLimitFilterRoot {
            context_id,
            config: FilterConfig::default(),
        })
    });
}

struct RateLimitFilterRoot {
    context_id: u32,
    config: FilterConfig,
}

impl RootContext for RateLimitFilterRoot {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        trace!("VM started");
        true
    }

    fn on_configure(&mut self, _config_size: usize) -> bool {
        //Check for the configuration passed by envoy.yaml
        let configuration: Vec<u8> = match self.get_configuration() {
            Some(c) => c,
            None => {
                info!("Configuration missing. Please check the envoy.yaml file for filter configuration");
                return false;
            }
        };

        // Parse and store the configuration passed by envoy.yaml
        match serde_json::from_slice::<FilterConfig>(configuration.as_ref()) {
            Ok(config) => {
                info!("configuring {}: {:?}", self.context_id, config);
                self.config = config;
            }
            Err(e) => {
                info!("Failed to parse envoy.yaml configuration: {:?}", e);
                return false;
            }
        }

        // Initialize the tick.
        self.set_tick_period(INITIALISATION_TICK);

        // Initialize the ratelimit cache and store in shared data.
        let ratelimit_reset_utc: DateTime<Utc> = self.get_ratelimit_reset();
        let ratelimit_cache: RateLimitCache = RateLimitCache {
            limit: self.config.ratelimit_limit,
            remaining: self.config.ratelimit_remaining,
            reset: ratelimit_reset_utc.format("%d/%m/%Y %T").to_string(),
        };
        let bytes: Vec<u8> = bincode::serialize(&ratelimit_cache).unwrap();
        self.set_shared_data(CACHE_KEY, Some(&bytes), None).unwrap();
        return true;
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(RateLimitFilter {
            //config: self.config.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn on_tick(&mut self) {
        self.set_tick_period(self.config.ratelimit_reset);
        match self.get_shared_data(CACHE_KEY) {
            (None, _) => info!("no ratelimit cache found"),
            (Some(data), _) => {
                // Reseting the ratelimit cache after every time window.
                let mut ratelimit_cache: RateLimitCache = bincode::deserialize(&data).unwrap();
                ratelimit_cache.remaining = ratelimit_cache.limit;
                let new_ratelimit_reset: DateTime<Utc> = self.get_ratelimit_reset();
                ratelimit_cache.reset = new_ratelimit_reset.format("%d/%m/%Y %T").to_string();
                let bytes: Vec<u8> = bincode::serialize(&ratelimit_cache).unwrap();
                self.set_shared_data(CACHE_KEY, Some(&bytes), None).unwrap();
                info!("reseted ratelimit for the next time window");
            }
        }
    }
}

impl RateLimitFilterRoot {
    // Next ratelimit quota window is calculated by adding the reset duration to current system time.
    fn get_ratelimit_reset(&self) -> DateTime<Utc> {
        let system_time = self.get_current_time();
        let next_quota_reset = system_time
            .checked_add(self.config.ratelimit_reset)
            .unwrap();
        let next_quota_reset_utc: DateTime<Utc> = next_quota_reset.into();
        return next_quota_reset_utc;
    }
}

impl Context for RateLimitFilterRoot {}

struct RateLimitFilter {
    //config: FilterConfig,
}

// Dummy rate limit cache representation
#[derive(Serialize, Deserialize, Debug)]
struct RateLimitCache {
    limit: u16,
    remaining: u16,
    reset: String,
}

impl HttpContext for RateLimitFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
        trace!("RateLimitFilter intercepted the HTTP request");
        match self.get_ratelimit_cache() {
            Some(cache) => {
                if cache.remaining > 0 {
                    // If not rate limited, proceed to the next filter after updating the counter.
                    info!("not rate limited. proceeding to the next filter ...");
                    self.update_ratelimit_cache();
                    return Action::Continue;
                } else {
                    // When rate limited, send a local reply with 429
                    info!("rate limited. closing connection with 429");
                    self.send_http_response(
                        429,
                        vec![
                            ("x-3scale-RateLimit-Remaining", &cache.remaining.to_string()),
                            ("x-3scale-RateLimit-Limit", &cache.limit.to_string()),
                            ("x-3scale-RateLimit-Reset", &cache.reset),
                        ],
                        None,
                    );
                    return Action::Pause;
                }
            }

            None => {
                info!("ratelimit cache does not exist.");
                return Action::Continue;
            }
        }
    }

    // Adding rate limit headers from the response path.
    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        match self.get_ratelimit_cache() {
            Some(cache) => {
                self.set_http_response_header(
                    "x-3scale-RateLimit-Limit",
                    Some(&cache.limit.to_string()),
                );
                self.set_http_response_header(
                    "x-3scale-RateLimit-Remaining",
                    Some(&cache.remaining.to_string()),
                );
                self.set_http_response_header("x-3scale-RateLimit-Reset", Some(&cache.reset));
                return Action::Continue;
            }

            None => {
                info!("ratelimit cache does not exist.");
                return Action::Continue;
            }
        }
    }
}

impl Context for RateLimitFilter {}

impl RateLimitFilter {
    fn get_ratelimit_cache(&self) -> Option<RateLimitCache> {
        match self.get_shared_data(CACHE_KEY) {
            (None, _) => return None,
            (Some(data), _) => {
                let ratelimit_cache: RateLimitCache = bincode::deserialize(&data).unwrap();
                trace!("cache : {}", ratelimit_cache.limit);
                return Some(ratelimit_cache);
            }
        }
    }

    fn update_ratelimit_cache(&self) -> bool {
        match self.get_shared_data(CACHE_KEY) {
            (None, _) => return false,
            (Some(data), _) => {
                let mut ratelimit_cache: RateLimitCache = bincode::deserialize(&data).unwrap();
                ratelimit_cache.remaining = ratelimit_cache.remaining - 1;
                let bytes: Vec<u8> = bincode::serialize(&ratelimit_cache).unwrap();
                self.set_shared_data(CACHE_KEY, Some(&bytes), None).unwrap();
                return true;
            }
        }
    }
}
