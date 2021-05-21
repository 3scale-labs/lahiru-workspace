use log::trace;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
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
        // self.add_http_request_header("x-3scale-service-key", "123456789");
        // self.add_http_request_header("x-3scale-application-key", "987654321");
        trace!("Metadata filter intercepted the HTTP request");
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_response_headers() {
            trace!("#{} <- {}: {}", self.context_id, name, value);
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        trace!("#{} completed.", self.context_id);
    }
}
