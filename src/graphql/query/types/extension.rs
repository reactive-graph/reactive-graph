use async_graphql::scalar;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::Extension;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphQLExtension {
    /// The name of the extension.
    pub name: String,

    /// The extension as JSON representation.
    pub extension: Value,
}
scalar!(GraphQLExtension, "Extension");

impl From<GraphQLExtension> for Extension {
    fn from(extension: GraphQLExtension) -> Self {
        Extension {
            name: extension.name.clone(),
            extension: extension.extension,
        }
    }
}

impl From<Extension> for GraphQLExtension {
    fn from(extension: Extension) -> Self {
        GraphQLExtension {
            name: extension.name.clone(),
            extension: extension.extension,
        }
    }
}
