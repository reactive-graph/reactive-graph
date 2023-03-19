use crate::HttpBody;
use http::Request;
use http::Response;
use http::Result;

#[derive(Debug)]
pub enum WebResourceProviderError {
    InitializationError,
}

pub trait WebResourceProvider: Send + Sync {
    /// The context path segment.
    fn get_context_path(&self) -> String;

    /// Handles a web resource.
    fn handle_web_resource(&self, path: String, request: Request<HttpBody>) -> Result<Response<HttpBody>>;
}

#[macro_export]
macro_rules! web_resource_provider {
    ($web_resource_provider:expr) => {{
        let web_resource_provider = $web_resource_provider.clone();
        let web_resource_provider: Result<Arc<dyn WebResourceProvider>, _> = <dyn query_interface::Object>::query_arc(web_resource_provider);
        if web_resource_provider.is_err() {
            return Err(WebResourceProviderError::InitializationError);
        }
        Ok(web_resource_provider.ok())
    }};
}
