use async_graphql::InputObject;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::NamespacedTypeParseError;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, InputObject)]
#[graphql(name = "ExtensionDefinition")]
pub struct GraphQLExtensionDefinition {
    /// The fully qualified namespace of the extension.
    #[graphql(name = "type")]
    pub _type: String,

    /// Optionally, the fully qualified namespace of the entity type.
    #[graphql(name = "entityType")]
    pub entity_type: Option<String>,

    /// The description of the extension.
    pub description: String,

    /// The extension as JSON representation.
    pub extension: Value,
}

impl TryFrom<GraphQLExtensionDefinition> for Extension {
    type Error = NamespacedTypeParseError;

    fn try_from(extension: GraphQLExtensionDefinition) -> Result<Self, Self::Error> {
        let extension_ty = ExtensionTypeId::from_str(&extension._type)?;
        let entity_ty = EntityTypeId::parse_optional_namespace(extension.entity_type)?;
        Ok(Extension {
            ty: extension_ty,
            entity_ty,
            description: extension.description.clone(),
            extension: extension.extension,
        })
    }
}

#[derive(Default)]
pub struct GraphQLExtensionDefinitions(Extensions);

impl GraphQLExtensionDefinitions {
    pub fn new(extensions: Extensions) -> Self {
        Self(extensions)
    }

    pub fn parse_definitions(extension_definitions: Vec<GraphQLExtensionDefinition>) -> Result<Extensions, NamespacedTypeParseError> {
        GraphQLExtensionDefinitions::try_from(extension_definitions).map(|e| e.0)
    }

    pub fn parse_optional_definitions(extension_definitions: Option<Vec<GraphQLExtensionDefinition>>) -> Result<Extensions, NamespacedTypeParseError> {
        match extension_definitions {
            Some(extension_definitions) => GraphQLExtensionDefinitions::try_from(extension_definitions).map(|extension_definition| extension_definition.0),
            None => Ok(Extensions::new()),
        }
    }

    // pub fn sorted(self) -> Vec<GraphQLExtension> {
    //     let mut extensions: Vec<GraphQLExtension> = self.into();
    //     extensions.sort();
    //     extensions
    // }
}

impl Deref for GraphQLExtensionDefinitions {
    type Target = Extensions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<GraphQLExtensionDefinitions> for Extensions {
    fn from(extensions: GraphQLExtensionDefinitions) -> Self {
        extensions.0
    }
}

impl TryFrom<Vec<GraphQLExtensionDefinition>> for GraphQLExtensionDefinitions {
    type Error = NamespacedTypeParseError;

    fn try_from(extension_definitions: Vec<GraphQLExtensionDefinition>) -> Result<Self, Self::Error> {
        let extensions = Extensions::new();
        for extension_definition in extension_definitions.into_iter() {
            extensions.push(Extension::try_from(extension_definition)?);
        }
        Ok(GraphQLExtensionDefinitions::new(extensions))
    }
}

// impl From<ExtensionDefinitions> for Vec<GraphQLExtension> {
//     fn from(extensions: ExtensionDefinitions) -> Self {
//         extensions.0.into_iter().map(|(_, extension)| extension.into()).collect()
//     }
// }
//
// impl From<Extensions> for ExtensionDefinitions {
//     fn from(extensions: Extensions) -> Self {
//         ExtensionDefinitions::new(extensions)
//     }
// }
//
//
// impl FromIterator<Extension> for ExtensionDefinitions {
//     fn from_iter<I: IntoIterator<Item = Extension>>(iter: I) -> Self {
//         let extensions = Extensions::new();
//         for extension in iter {
//             extensions.push(extension.clone());
//         }
//         ExtensionDefinitions::new(extensions)
//     }
// }
