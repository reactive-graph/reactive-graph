use std::cmp::Ordering;

use async_graphql::InputObject;
use async_graphql::Object;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::graphql::mutation::ExtensionTypeIdDefinition;
use crate::model::Extension;
use crate::model::NamespacedTypeGetter;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, InputObject)]
#[graphql(name = "ExtensionDefinition")]
pub struct GraphQLExtension {
    /// The namespace of the extension.
    #[graphql(name = "type")]
    pub ty: ExtensionTypeIdDefinition,

    /// The description of the extension.
    pub description: String,

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
    async fn namespace(&self) -> String {
        self.ty.namespace.clone()
    }

    /// The name of the extension.
    async fn name(&self) -> String {
        self.ty.type_name.clone()
    }

    /// The name of the extension.
    async fn description(&self) -> String {
        self.description.clone()
    }

    /// The additional information as JSON representation (schema-less).
    async fn extension(&self) -> Value {
        self.extension.clone()
    }
}

impl From<GraphQLExtension> for Extension {
    fn from(extension: GraphQLExtension) -> Self {
        Extension {
            ty: extension.ty.into(),
            description: extension.description.clone(),
            extension: extension.extension,
        }
    }
}

impl From<Extension> for GraphQLExtension {
    fn from(extension: Extension) -> Self {
        GraphQLExtension {
            ty: ExtensionTypeIdDefinition {
                namespace: extension.namespace(),
                type_name: extension.type_name(),
            },
            description: extension.description.clone(),
            extension: extension.extension,
        }
    }
}

impl From<&Extension> for GraphQLExtension {
    fn from(extension: &Extension) -> Self {
        GraphQLExtension {
            ty: ExtensionTypeIdDefinition {
                namespace: extension.namespace(),
                type_name: extension.type_name(),
            },
            description: extension.description.clone(),
            extension: extension.extension.clone(),
        }
    }
}

impl PartialOrd<Self> for GraphQLExtension {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GraphQLExtension {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ty.cmp(&other.ty)
    }
}
