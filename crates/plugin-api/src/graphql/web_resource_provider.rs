use async_trait::async_trait;
use http::Request;
use http::Response;
use http::Result;
use uuid::Uuid;

use crate::injectable;
use crate::HttpBody;

#[async_trait]
#[injectable]
pub trait WebResourceProvider: Send + Sync {
    fn id(&self) -> Uuid;

    /// The context path segment.
    fn get_context_path(&self) -> String;

    /// Handles a web resource.
    async fn handle_web_resource(&self, path: String, request: Request<HttpBody>) -> Result<Response<HttpBody>>;
}
