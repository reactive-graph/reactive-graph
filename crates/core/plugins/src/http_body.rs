use serde_json::Value;

pub enum HttpBody {
    None,
    Binary(Vec<u8>),
    Json(Value),
    PlainText(String),
}
