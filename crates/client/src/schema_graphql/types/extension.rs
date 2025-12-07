use crate::schema_graphql::scalar::Json;
use crate::schema_graphql::types::entity_type::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::InvalidExtensionError;
use reactive_graph_graph::NamespacedTypeGetter;
use serde_json::Value;
use std::ops::Deref;
use std::str::FromStr;

#[derive(cynic::InputObject, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct ExtensionDefinition {
    // The fully qualified namespace of the extension.
    #[cynic(rename = "type")]
    pub _type: String,
    // The fully qualified namespace of the entity type which is the type constraint of the extension.
    pub entity_type: Option<String>,
    // The description of the extension.
    pub description: String,
    // The extension data.
    pub extension: Json,
}

impl From<reactive_graph_graph::Extension> for ExtensionDefinition {
    fn from(extension: reactive_graph_graph::Extension) -> Self {
        ExtensionDefinition {
            _type: extension.ty.namespace().to_string(),
            entity_type: extension.entity_ty.map(|entity_ty| entity_ty.namespace().to_string()),
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

impl From<reactive_graph_graph::Extensions> for ExtensionDefinitions {
    fn from(extensions: reactive_graph_graph::Extensions) -> Self {
        extensions.into_iter().map(|(_, extension)| extension).collect()
    }
}

impl FromIterator<reactive_graph_graph::Extension> for ExtensionDefinitions {
    fn from_iter<I: IntoIterator<Item = reactive_graph_graph::Extension>>(iter: I) -> Self {
        let mut extensions = ExtensionDefinitions::new();
        for extension in iter {
            extensions.0.push(extension.into());
        }
        extensions
    }
}

impl Default for ExtensionDefinitions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct Extension {
    /// The fully qualified namespace of the extension.
    #[cynic(rename = "type")]
    pub _type: String,

    /// The fully qualified namespace of the entity type which is the type constraint of the extension.
    pub entity_type: Option<EntityType>,

    /// Textual description of the extension.
    pub description: String,

    /// The extension as JSON representation.
    pub extension: Value,
}

impl TryFrom<Extension> for reactive_graph_graph::Extension {
    type Error = InvalidExtensionError;

    fn try_from(extension: Extension) -> Result<Self, Self::Error> {
        Ok(reactive_graph_graph::Extension {
            ty: ExtensionTypeId::from_str(&extension._type).map_err(InvalidExtensionError::InvalidExtension)?,
            entity_ty: match extension.entity_type {
                None => None,
                Some(entity_type) => Some(EntityTypeId::from_str(&entity_type._type).map_err(InvalidExtensionError::InvalidEntityType)?),
            },
            description: extension.description,
            extension: extension.extension,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Extensions(pub Vec<Extension>);

impl Deref for Extensions {
    type Target = Vec<Extension>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<Extensions> for reactive_graph_graph::Extensions {
    type Error = InvalidExtensionError;

    fn try_from(extensions: Extensions) -> Result<Self, Self::Error> {
        let extensions_2 = reactive_graph_graph::Extensions::new();
        for extension in extensions.0 {
            extensions_2.push(reactive_graph_graph::Extension::try_from(extension)?);
        }
        Ok(extensions_2)
    }
}
