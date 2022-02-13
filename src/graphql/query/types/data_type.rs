use async_graphql::*;
use serde::{Deserialize, Serialize};
use strum::Display;

/// The data types of a value.
///
/// Derived from serde_json::Value but without value payload.
#[derive(Enum, Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Display)]
#[serde(rename_all = "lowercase")]
#[graphql(name = "DataType", remote = "inexor_rgf_core_model::DataType")]
pub enum GraphQLDataType {
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
