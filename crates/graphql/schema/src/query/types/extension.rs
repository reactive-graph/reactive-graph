use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::cmp::Ordering;
use std::ops::Deref;
use std::sync::Arc;

use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::Extension;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_type_system_api::EntityTypeManager;

use crate::query::GraphQLEntityType;
use crate::query::GraphQLNamespacedType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GraphQLExtension {
    pub extension: Extension,
}

/// An extension provides named but schema-less additional information.
/// Entity types, relation types and property types can provide additional
/// metadata. For example an extension named "shape" provides information
/// about the look and feel in the flow editor.
#[Object(name = "Extension")]
impl GraphQLExtension {
    /// The fully qualified namespace of the extension.
    #[graphql(name = "type")]
    async fn ty(&self) -> String {
        self.extension.namespace().to_string()
    }

    /// The namespaced type.
    async fn namespaced_type(&self) -> GraphQLNamespacedType {
        self.extension.namespaced_type().into()
    }

    /// The entity type constraint of the extension.
    #[graphql(name = "entityType")]
    async fn entity_type(&self, context: &Context<'_>) -> Result<Option<GraphQLEntityType>> {
        let Some(entity_ty) = self.extension.entity_ty.clone() else {
            return Ok(None);
        };
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let entity_ty: EntityTypeId = entity_ty.into();
        match entity_type_manager.get(&entity_ty) {
            None => Err(format!("Entity type not found: {:?}", entity_ty).into()),
            Some(entity_type) => Ok(Some(entity_type.into())),
        }
    }

    /// The name of the extension.
    async fn description(&self) -> String {
        self.extension.description.clone()
    }

    /// The additional information as JSON representation (schema-less).
    async fn extension(&self) -> Value {
        self.extension.extension.clone()
    }
}

impl From<Extension> for GraphQLExtension {
    fn from(extension: Extension) -> Self {
        GraphQLExtension { extension }
    }
}

impl From<GraphQLExtension> for Extension {
    fn from(extension: GraphQLExtension) -> Self {
        extension.extension
    }
}

impl From<&Extension> for GraphQLExtension {
    fn from(extension: &Extension) -> Self {
        GraphQLExtension { extension: extension.clone() }
    }
}

impl PartialOrd<Self> for GraphQLExtension {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GraphQLExtension {
    fn cmp(&self, other: &Self) -> Ordering {
        self.extension.ty.cmp(&other.extension.ty)
    }
}

#[derive(Default)]
pub struct GraphQLExtensions(Extensions);

impl GraphQLExtensions {
    pub fn new(extensions: Extensions) -> Self {
        Self(extensions)
    }

    pub fn sorted(self) -> Vec<GraphQLExtension> {
        let mut extensions: Vec<GraphQLExtension> = self.into();
        extensions.sort();
        extensions
    }
}

impl Deref for GraphQLExtensions {
    type Target = Extensions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<GraphQLExtension>> for GraphQLExtensions {
    fn from(extensions: Vec<GraphQLExtension>) -> Self {
        let extensions: Extensions = extensions.into_iter().map(|extension| extension.into()).collect();
        GraphQLExtensions::new(extensions)
    }
}

impl From<GraphQLExtensions> for Vec<GraphQLExtension> {
    fn from(extensions: GraphQLExtensions) -> Self {
        extensions.0.into_iter().map(|(_, extension)| extension.into()).collect()
    }
}

impl From<Extensions> for GraphQLExtensions {
    fn from(extensions: Extensions) -> Self {
        GraphQLExtensions::new(extensions)
    }
}

impl From<GraphQLExtensions> for Extensions {
    fn from(extensions: GraphQLExtensions) -> Self {
        extensions.0
    }
}

impl FromIterator<Extension> for GraphQLExtensions {
    fn from_iter<I: IntoIterator<Item = Extension>>(iter: I) -> Self {
        let extensions = Extensions::new();
        for extension in iter {
            extensions.push(extension.clone());
        }
        GraphQLExtensions::new(extensions)
    }
}
