use std::sync::Arc;

use async_graphql::*;

use crate::api::ComponentManager;
use crate::api::EntityBehaviourRegistry;
use crate::api::EntityTypeManager;
use crate::api::RelationTypeManager;
use crate::graphql::query::GraphQLComponent;
use crate::graphql::query::GraphQLEntityBehaviour;
use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::GraphQLPropertyType;
use crate::graphql::query::GraphQLRelationType;
use crate::model::EntityType;
use crate::model::NamespacedTypeGetter;

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
        match context.data::<Arc<dyn ComponentManager>>() {
            Ok(component_manager) => self
                .entity_type
                .components
                .iter()
                .filter_map(|component_name| component_manager.get(component_name))
                .map(|component| component.into())
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// The properties / property types which are defined by the entity type or
    /// by one of the components.
    async fn properties(&self, name: Option<String>) -> Vec<GraphQLPropertyType> {
        match name {
            Some(name) => self
                .entity_type
                .properties
                .to_vec()
                .iter()
                .filter(|property_type| property_type.name == name.clone())
                .cloned()
                .map(|property_type| property_type.into())
                .collect(),
            None => self.entity_type.properties.iter().cloned().map(|property_type| property_type.into()).collect(),
        }
    }

    /// The extensions which are defined by the entity type.
    async fn extensions(&self, name: Option<String>) -> Vec<GraphQLExtension> {
        match name {
            Some(name) => self
                .entity_type
                .extensions
                .to_vec()
                .iter()
                .filter(|extension| extension.name == name.clone())
                .cloned()
                .map(|extension| extension.into())
                .collect(),
            None => self.entity_type.extensions.iter().cloned().map(|extension| extension.into()).collect(),
        }
    }

    /// List of relation types which has this entity type as outbound.
    async fn outbound_relations(&self, context: &Context<'_>) -> Vec<GraphQLRelationType> {
        match context.data::<Arc<dyn RelationTypeManager>>() {
            Ok(relation_type_manager) => relation_type_manager
                .get_all()
                .iter()
                .filter(|relation_type| {
                    relation_type.outbound_type.type_name() == "*"
                        || relation_type.outbound_type.eq_entity_type(&self.entity_type.ty)
                        || self
                            .entity_type
                            .components
                            .iter()
                            .any(|component_ty| relation_type.outbound_type.eq_component(component_ty))
                })
                .map(|relation_type| relation_type.clone().into())
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// List of relation types which has this entity type as inbound.
    async fn inbound_relations(&self, context: &Context<'_>) -> Vec<GraphQLRelationType> {
        match context.data::<Arc<dyn RelationTypeManager>>() {
            Ok(relation_type_manager) => relation_type_manager
                .get_all()
                .iter()
                .filter(|relation_type| {
                    relation_type.inbound_type.type_name() == "*"
                        || relation_type.inbound_type.eq_entity_type(&self.entity_type.ty)
                        || self
                            .entity_type
                            .components
                            .iter()
                            .any(|component_ty| relation_type.inbound_type.eq_component(component_ty))
                })
                .map(|relation_type| relation_type.clone().into())
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Returns true, if the entity type is valid. This means all components exists.
    async fn is_valid(&self, context: &Context<'_>) -> bool {
        match context.data::<Arc<dyn EntityTypeManager>>() {
            Ok(entity_type_manager) => entity_type_manager.validate(&self.entity_type.ty),
            Err(_) => false,
        }
    }

    async fn behaviours(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityBehaviour>> {
        let entity_behaviour_registry = context.data::<Arc<dyn EntityBehaviourRegistry>>()?;
        let entity_behaviour_types = entity_behaviour_registry
            .get_behaviour_types(&self.entity_type.ty)
            .iter()
            .map(|entity_behaviour_ty| GraphQLEntityBehaviour::from(entity_behaviour_ty.clone()))
            .collect();
        Ok(entity_behaviour_types)
    }
}

impl From<EntityType> for GraphQLEntityType {
    fn from(entity_type: EntityType) -> Self {
        GraphQLEntityType { entity_type }
    }
}
