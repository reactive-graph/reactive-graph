pub mod create_random_entity_instance;
pub use create_random_entity_instance::*;

use random_string::generate;
use serde_json::Value;
use serde_json::json;

const CHARSET_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn r_string() -> String {
    generate(10, CHARSET_LETTERS).to_string()
}

pub fn r_json_string() -> Value {
    json!(r_string())
}
