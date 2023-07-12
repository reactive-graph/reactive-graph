use core::fmt;
use schemars::JsonSchema;
use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

/// The mutability of a property.
#[derive(Clone, Debug, PartialEq, Copy, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Mutability {
    /// The property is mutable.
    Mutable,

    /// The property is immutable.
    Immutable,
}

impl Mutability {
    pub fn mutable() -> Self {
        Mutability::Mutable
    }
    pub fn immutable() -> Self {
        Mutability::Immutable
    }
}

impl From<&str> for Mutability {
    fn from(value: &str) -> Self {
        return match value.to_lowercase().as_str() {
            "mutable" => Self::Mutable,
            "immutable" => Self::Immutable,
            _ => Self::Mutable,
        };
    }
}

impl Display for Mutability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
