use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::str;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(MetadataRoot) });
}

struct MetadataRoot;

impl Context for MetadataRoot {}

impl RootContext for MetadataRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MetadataFilter { context_id }))
    }
}

struct MetadataFilter {
    context_id: u32,
}

impl Context for MetadataFilter {}

impl HttpContext for MetadataFilter {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_request_headers() {
            info!("#{} -> {}: {}", self.context_id, name, value);
        }
        info!("Metadata filter intercepted the HTTP request");
        let service_key_utf8 = self
            .get_property(vec!["metadata", "filter_metadata", "3scale", "service_key"])
            .unwrap();
        let service_key = match str::from_utf8(&service_key_utf8) {
            Ok(sk) => sk,
            Err(e) => panic!("Error : {}", e),
        };
        let application_key_utf8 = self
            .get_property(vec![
                "metadata",
                "filter_metadata",
                "3scale",
                "application_key",
            ])
            .unwrap();
        let application_key = match str::from_utf8(&application_key_utf8) {
            Ok(ak) => ak,
            Err(e) => panic!("Error: {}", e),
        };
        info!("service key from dynamic metadata: {}", service_key);
        info!("application key from dynamic metadata: {}", application_key);
        Action::Continue
    }

    fn on_log(&mut self) {
        info!("#{} completed.", self.context_id);
    }
}
