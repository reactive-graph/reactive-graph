use async_graphql::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Extension on a type. The extension allows to extend information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Extension {
    /// The name of the extension.
    pub name: String,

    /// The extension as JSON representation.
    pub extension: Value,
}
scalar!(Extension);
