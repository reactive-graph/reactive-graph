pub mod create_random_entity_instance;
pub use create_random_entity_instance::*;

pub mod create_default_connector;
pub use create_default_connector::*;

pub mod create_relation_instance_with_properties;
pub use create_relation_instance_with_properties::*;

use random_string::generate;
use serde_json::{Value, json};

const CHARSET_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn r_string() -> String {
    generate(10, CHARSET_LETTERS).to_string()
}

pub fn r_json_string() -> Value {
    json!(r_string())
}
