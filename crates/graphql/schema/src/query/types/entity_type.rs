use std::ops::Deref;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use reactive_graph_behaviour_service_api::EntityBehaviourRegistry;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypes;
use reactive_graph_graph::JsonSchemaIdGetter;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionJsonSchemaGetter;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;
use serde_json::Value;

use crate::mutation::ExtensionTypeIdDefinition;
use crate::query::GraphQLComponent;
use crate::query::GraphQLEntityBehaviour;
use crate::query::GraphQLExtension;
use crate::query::GraphQLPropertyType;
use crate::query::GraphQLRelationType;

pub struct GraphQLEntityType {
    entity_type: EntityType,
}

/// Entity types defines the type of entity instance.
#[Object(name = "EntityType")]
impl GraphQLEntityType {
    /// The namespace the entity type belongs to.
    async fn namespace(&self) -> String {
        self.entity_type.namespace()
    }

    /// The name of the entity type.
    ///
    /// The name is the unique identifier for entity types.
    async fn name(&self) -> String {
        self.entity_type.type_name()
    }

    /// Textual description of the entity type.
    async fn description(&self) -> String {
        self.entity_type.description.clone()
    }

    /// The components of the entity type.
    async fn components(&self, context: &Context<'_>) -> Vec<GraphQLComponent> {
        match context.data::<Arc<dyn ComponentManager + Send + Sync>>() {
            Ok(component_manager) => self
                .entity_type
                .components
                .iter()
                .filter_map(|component_ty| component_manager.get(&component_ty))
                .map(|component| component.into())
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// The count of components.
    async fn count_components(&self) -> usize {
        self.entity_type.components.len()
    }

    /// The properties / property types which are defined by the entity type or
    /// by one of the components.
    async fn properties(
        &self,
        #[graphql(desc = "The name of the property")] name: Option<String>,
        #[graphql(desc = "If true, the properties are sorted by name")] sort: Option<bool>,
    ) -> Vec<GraphQLPropertyType> {
        match name {
            Some(name) => self
                .entity_type
                .properties
                .iter()
                .filter(|property_type| property_type.name == name.clone())
                // .cloned()
                .map(|property_type| property_type.value().into())
                .collect(),
            None => {
                let mut properties: Vec<GraphQLPropertyType> = self.entity_type.properties.iter().map(|property_type| property_type.value().into()).collect();
                if sort.unwrap_or_default() {
                    properties.sort();
                }
                properties
            }
        }
    }

    /// The count of properties.
    async fn count_properties(&self) -> usize {
        self.entity_type.properties.len()
    }

    /// The extensions which are defined by the entity type.
    async fn extensions(
        &self,
        #[graphql(name = "type")] extension_ty: Option<ExtensionTypeIdDefinition>,
        #[graphql(desc = "If true, the extensions are sorted by type")] sort: Option<bool>,
    ) -> Vec<GraphQLExtension> {
        match extension_ty {
            Some(extension_ty) => {
                let extension_ty = extension_ty.into();
                self.entity_type
                    .extensions
                    .iter()
                    .filter(|extension| extension.ty == extension_ty)
                    .map(|extension| extension.value().into())
                    .collect()
            }
            None => {
                let mut extensions: Vec<GraphQLExtension> = self.entity_type.extensions.iter().map(|extension| extension.value().into()).collect();
                if sort.unwrap_or_default() {
                    extensions.sort();
                }
                extensions
            }
        }
    }

    /// The count of extensions.
    async fn count_extensions(&self) -> usize {
        self.entity_type.extensions.len()
    }

    /// List of relation types which has this entity type as outbound.
    async fn outbound_relations(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationType>> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_types = relation_type_manager
            .get_all()
            .iter()
            .filter(|relation_type| {
                relation_type.outbound_type.type_name() == "*"
                    || relation_type.outbound_type.eq_entity_type(&self.entity_type.ty)
                    || self
                        .entity_type
                        .components
                        .iter()
                        .any(|component_ty| relation_type.outbound_type.eq_component(&component_ty))
            })
            .map(|relation_type| relation_type.clone().into())
            .collect();
        Ok(relation_types)
    }

    async fn count_outbound_relations(&self, context: &Context<'_>) -> Result<usize> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let count = relation_type_manager
            .get_all()
            .iter()
            .filter(|relation_type| {
                relation_type.outbound_type.type_name() == "*"
                    || relation_type.outbound_type.eq_entity_type(&self.entity_type.ty)
                    || self
                        .entity_type
                        .components
                        .iter()
                        .any(|component_ty| relation_type.outbound_type.eq_component(&component_ty))
            })
            .count();
        Ok(count)
    }

    /// List of relation types which has this entity type as inbound.
    async fn inbound_relations(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationType>> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_types = relation_type_manager
            .get_all()
            .iter()
            .filter(|relation_type| {
                relation_type.inbound_type.type_name() == "*"
                    || relation_type.inbound_type.eq_entity_type(&self.entity_type.ty)
                    || self
                        .entity_type
                        .components
                        .iter()
                        .any(|component_ty| relation_type.inbound_type.eq_component(&component_ty))
            })
            .map(|relation_type| relation_type.clone().into())
            .collect();
        Ok(relation_types)
    }

    async fn count_inbound_relations(&self, context: &Context<'_>) -> Result<usize> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let count = relation_type_manager
            .get_all()
            .iter()
            .filter(|relation_type| {
                relation_type.inbound_type.type_name() == "*"
                    || relation_type.inbound_type.eq_entity_type(&self.entity_type.ty)
                    || self
                        .entity_type
                        .components
                        .iter()
                        .any(|component_ty| relation_type.inbound_type.eq_component(&component_ty))
            })
            .count();
        Ok(count)
    }

    /// Returns true, if the entity type is valid. This means all components exists.
    async fn is_valid(&self, context: &Context<'_>) -> bool {
        match context.data::<Arc<dyn EntityTypeManager + Send + Sync>>() {
            Ok(entity_type_manager) => entity_type_manager.validate(&self.entity_type.ty),
            Err(_) => false,
        }
    }

    async fn behaviours(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityBehaviour>> {
        let entity_behaviour_registry = context.data::<Arc<dyn EntityBehaviourRegistry + Send + Sync>>()?;
        let entity_behaviour_types = entity_behaviour_registry
            .get_behaviour_types(&self.entity_type.ty)
            .iter()
            .map(|entity_behaviour_ty| GraphQLEntityBehaviour::from(entity_behaviour_ty.clone()))
            .collect();
        Ok(entity_behaviour_types)
    }

    /// Returns the JSON schema of the entity type.
    async fn json_schema(&self) -> Value {
        self.entity_type.json_schema().to_value()
    }

    /// Returns the JSON schema identifier ($id) of the entity type.
    async fn json_schema_id(&self) -> String {
        self.entity_type.json_schema_id().to_string()
    }
}

impl From<EntityType> for GraphQLEntityType {
    fn from(entity_type: EntityType) -> Self {
        GraphQLEntityType { entity_type }
    }
}

pub struct GraphQLEntityTypes(Vec<GraphQLEntityType>);

impl Deref for GraphQLEntityTypes {
    type Target = Vec<GraphQLEntityType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<GraphQLEntityTypes> for Vec<GraphQLEntityType> {
    fn from(value: GraphQLEntityTypes) -> Self {
        value.0
    }
}

impl From<EntityTypes> for GraphQLEntityTypes {
    fn from(entity_types: EntityTypes) -> Self {
        let entity_types = entity_types.into_iter().map(|(_, entity_type)| entity_type.into()).collect();
        GraphQLEntityTypes(entity_types)
    }
}
