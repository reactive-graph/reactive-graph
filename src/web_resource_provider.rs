use http::{Request, Response, Result};
use serde_json::Value;

enum ResponseBody {
    None,
    Binary(Vec<u8>),
    Json(Value),
    PlainText(String),
}

pub trait WebResourceProvider: Send + Sync {
    /// The base path segment.
    fn get_base_path(&self) -> String;

    /// The base path segment.
    fn handle_web_resource(&self, path: String) -> Result<Response<ResponseBody>>;
}
