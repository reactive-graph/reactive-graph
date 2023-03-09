use core::fmt;
use fmt::Display;
use std::collections::HashMap;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

/// Derived from serde_json::Value but without value payload.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    /// Represents a JSON null value.
    Null,

    /// Represents a JSON boolean.
    Bool,

    /// Represents a JSON number, whether integer or floating point.
    Number,

    /// Represents a JSON string.
    String,

    /// Represents a JSON array.
    Array,

    /// Represents a JSON object.
    Object,

    /// Represents any type (relations).
    Any,
}

impl DataType {
    pub fn bool() -> Self {
        DataType::Bool
    }

    pub fn number() -> Self {
        DataType::Number
    }

    pub fn string() -> Self {
        DataType::String
    }

    pub fn default_value(&self) -> Value {
        match self {
            DataType::Bool => json!(false),
            DataType::Number => json!(0),
            DataType::String => json!(""),
            DataType::Array => json!(Vec::<Value>::new()),
            DataType::Object => json!(HashMap::<String, Value>::new()),
            _ => json!(""),
        }
    }
}

impl From<&str> for DataType {
    fn from(value: &str) -> Self {
        return match value.to_lowercase().as_str() {
            "bool" => Self::Bool,
            "number" => Self::Number,
            "string" => Self::String,
            "array" => Self::Array,
            "object" => Self::Object,
            "any" => Self::Any,
            _ => Self::String,
        };
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
