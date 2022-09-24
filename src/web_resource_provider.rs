use crate::HttpBody;
use http::Request;
use http::Response;
use http::Result;

#[derive(Debug)]
pub enum WebResourceProviderError {
    InitializationError,
}

pub trait WebResourceProvider: Send + Sync {
    /// The base path segment.
    fn get_base_path(&self) -> String;

    /// Handles a web resource.
    fn handle_web_resource(&self, path: String, request: Request<HttpBody>) -> Result<Response<HttpBody>>;
}
