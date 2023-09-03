use serde_json::Value;

use crate::model::NamespacedTypeGetter;
use crate::schema::scalar::Json;

#[derive(cynic::InputObject, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct ExtensionTypeId {
    pub name: String,
    pub namespace: String,
}

impl From<crate::model::ExtensionTypeId> for ExtensionTypeId {
    fn from(ty: crate::model::ExtensionTypeId) -> Self {
        ExtensionTypeId {
            name: ty.type_name(),
            namespace: ty.namespace(),
        }
    }
}

#[derive(cynic::InputObject, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct ExtensionDefinition {
    #[cynic(rename = "type")]
    pub type_: ExtensionTypeId,
    pub description: String,
    pub extension: Json,
}

impl From<crate::model::Extension> for ExtensionDefinition {
    fn from(extension: crate::model::Extension) -> Self {
        ExtensionDefinition {
            type_: extension.ty.into(),
            description: extension.description,
            extension: extension.extension.into(),
        }
    }
}

pub struct ExtensionDefinitions(pub Vec<ExtensionDefinition>);

impl ExtensionDefinitions {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<ExtensionDefinitions> for Vec<ExtensionDefinition> {
    fn from(extensions: ExtensionDefinitions) -> Self {
        extensions.0.into_iter().collect()
    }
}

impl From<crate::model::Extensions> for ExtensionDefinitions {
    fn from(extensions: crate::model::Extensions) -> Self {
        extensions.into_iter().map(|(_, extension) | extension).collect()
    }
}

impl FromIterator<crate::model::Extension> for ExtensionDefinitions {
    fn from_iter<I: IntoIterator<Item=crate::model::Extension>>(iter: I) -> Self {
        let mut extensions = ExtensionDefinitions::new();
        for extension in iter {
            extensions.0.push(extension.into());
        }
        extensions
    }
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct Extension {
    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    pub description: String,

    /// The extension as JSON representation.
    pub extension: Value,
}

impl From<Extension> for crate::model::Extension {
    fn from(extension: Extension) -> Self {
        let ty = crate::model::ExtensionTypeId::new_from_type(extension.namespace, extension.name);
        crate::model::Extension {
            ty,
            description: extension.description,
            extension: extension.extension,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Extensions(pub Vec<Extension>);

impl From<Extensions> for crate::model::Extensions {
    fn from(extensions: Extensions) -> Self {
        extensions.0.into_iter().map(|extension| extension.into()).collect()
    }
}
