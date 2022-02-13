use async_graphql::{InputObject, Object};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::Extension;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "ExtensionDefinition")]
pub struct GraphQLExtension {
    /// The name of the extension.
    pub name: String,

    /// The extension as JSON representation.
    pub extension: Value,
}

/// An extension provides named but schema-less additional information.
/// Entity types, relation types and property types can provide additional
/// meta data. For example an extension named "shape" provides information
/// about the look and feel in the flow editor.
#[Object(name = "Extension")]
impl GraphQLExtension {
    /// The name of the extension.
    async fn name(&self) -> String {
        self.name.clone()
    }

    /// The additional information as JSON representation (schema-less).
    async fn extension(&self) -> Value {
        self.extension.clone()
    }
}

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
