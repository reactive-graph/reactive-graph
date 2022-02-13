use std::sync::Arc;

use async_graphql::*;

use crate::api::{ComponentManager, RelationTypeManager};
use crate::graphql::query::{GraphQLComponent, GraphQLExtension, GraphQLPropertyType, GraphQLRelationType};
use crate::model::EntityType;

pub struct GraphQLEntityType {
    entity_type: EntityType,
}

/// Entity types defines the type of entity instance.
#[Object(name = "EntityType")]
impl GraphQLEntityType {
    /// The name of the entity type.
    ///
    /// The name is the unique identifier for entity types.
    async fn name(&self) -> String {
        self.entity_type.name.clone()
    }

    /// The entity type belongs to the given group of entity types.
    async fn group(&self) -> String {
        self.entity_type.group.clone()
    }

    /// Textual description of the entity type.
    async fn description(&self) -> String {
        self.entity_type.description.clone()
    }

    /// The components of the entity type.
    async fn components(&self, context: &Context<'_>) -> Vec<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>();
        if component_manager.is_ok() {
            let component_manager = component_manager.unwrap();
            self.entity_type
                .components
                .iter()
                .filter_map(|component| component_manager.get(component.clone()))
                .map(|component| component.into())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// The behaviours.
    async fn behaviours(&self) -> Vec<String> {
        self.entity_type.behaviours.to_vec()
    }

    /// The properties / property types which are defined by the entity type or
    /// by one of the components.
    async fn properties(&self, name: Option<String>) -> Vec<GraphQLPropertyType> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .entity_type
                .properties
                .to_vec()
                .iter()
                .filter(|property_type| property_type.name == name.clone())
                .cloned()
                .map(|property_type| property_type.into())
                .collect();
        }
        self.entity_type.properties.iter().cloned().map(|property_type| property_type.into()).collect()
    }

    /// The extensions which are defined by the entity type.
    async fn extensions(&self, name: Option<String>) -> Vec<GraphQLExtension> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .entity_type
                .extensions
                .to_vec()
                .iter()
                .filter(|extension| extension.name == name.clone())
                .cloned()
                .map(|extension| extension.into())
                .collect();
        }
        self.entity_type.extensions.iter().cloned().map(|extension| extension.into()).collect()
    }

    /// List of relation types which has this entity type as outbound.
    async fn outbound_relations(&self, context: &Context<'_>) -> Vec<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>();
        if relation_type_manager.is_ok() {
            let relation_type_manager = relation_type_manager.unwrap();
            return relation_type_manager
                .get_relation_types()
                .iter()
                .filter(|relation_type| relation_type.outbound_type.clone() == "*" || relation_type.outbound_type.clone() == self.entity_type.name.clone())
                .map(|relation_type| relation_type.clone().into())
                .collect();
        }
        Vec::new()
    }

    /// List of relation types which has this entity type as inbound.
    async fn inbound_relations(&self, context: &Context<'_>) -> Vec<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>();
        if relation_type_manager.is_ok() {
            let relation_type_manager = relation_type_manager.unwrap();
            return relation_type_manager
                .get_relation_types()
                .iter()
                .filter(|relation_type| relation_type.inbound_type.clone() == "*" || relation_type.inbound_type.clone() == self.entity_type.name.clone())
                .map(|relation_type| relation_type.clone().into())
                .collect();
        }
        Vec::new()
    }
}

impl From<EntityType> for GraphQLEntityType {
    fn from(entity_type: EntityType) -> Self {
        GraphQLEntityType { entity_type }
    }
}
