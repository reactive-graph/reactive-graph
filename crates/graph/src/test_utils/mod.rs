pub use default_from::DefaultFrom;

use random_string::generate;
use serde_json::Value;
use serde_json::json;

pub mod default_from;

const CHARSET_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn r_string() -> String {
    generate(10, CHARSET_LETTERS).to_string()
}

pub fn r_string_255() -> String {
    generate(255, CHARSET_LETTERS).to_string()
}

pub fn r_string_1000() -> String {
    generate(1000, CHARSET_LETTERS).to_string()
}

pub fn r_json_string() -> Value {
    json!(r_string())
}
