use std::sync::Arc;

use async_graphql::*;

use crate::api::{ComponentManager, EntityTypeManager};
use crate::graphql::query::GraphQLEntityType;
use crate::model::{Component, Extension, PropertyType, RelationType};

pub struct GraphQLRelationType {
    relation_type: RelationType,
}

/// A relation type defines the type of an relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also the relation type defines the properties of the relation instance.
#[Object(name = "RelationType")]
impl GraphQLRelationType {
    /// The outbound entity type.
    async fn outbound_types(&self, context: &Context<'_>) -> Vec<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>();
        if entity_type_manager.is_ok() {
            let entity_type_manager = entity_type_manager.unwrap();
            if self.relation_type.outbound_type == "*" {
                return entity_type_manager
                    .get_entity_types()
                    .iter()
                    .map(|entity_type| entity_type.clone().into())
                    .collect();
            }
            if let Some(entity_type) = entity_type_manager.get(self.relation_type.outbound_type.clone()) {
                let mut outbound_types = Vec::new();
                outbound_types.push(entity_type.clone().into());
                return outbound_types;
            }
        }
        Vec::new()
    }

    /// The name of the relation type.
    ///
    /// The name is the unique identifier for relation types.
    ///
    /// Returns "default_connector" for "default_connector__property_name__property_name"
    /// (without type suffix).
    async fn name(&self) -> String {
        self.relation_type.type_name.clone()
    }

    /// The full name of the relation type.
    ///
    /// Returns "default_connector__property_name__property_name" (with type suffix).
    async fn full_name(&self) -> String {
        self.relation_type.full_name.clone()
    }

    /// The inbound entity type.
    async fn inbound_types(&self, context: &Context<'_>) -> Vec<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>();
        if entity_type_manager.is_ok() {
            let entity_type_manager = entity_type_manager.unwrap();
            if self.relation_type.inbound_type == "*" {
                return entity_type_manager
                    .get_entity_types()
                    .iter()
                    .map(|entity_type| entity_type.clone().into())
                    .collect();
            }
            if let Some(entity_type) = entity_type_manager.get(self.relation_type.inbound_type.clone()) {
                let mut inbound_types = Vec::new();
                inbound_types.push(entity_type.clone().into());
                return inbound_types;
            }
        }
        Vec::new()
    }

    /// The relation type belongs to the given group of relation types.
    async fn group(&self) -> String {
        self.relation_type.group.clone()
    }

    /// Textual description of the relation type.
    async fn description(&self) -> String {
        self.relation_type.description.clone()
    }

    /// The relation type composes it's properties by these components.
    async fn components(&self, context: &Context<'_>) -> Vec<Component> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>();
        if component_manager.is_ok() {
            let component_manager = component_manager.unwrap();
            self.relation_type
                .components
                .iter()
                .filter_map(|component| component_manager.get(component.clone()))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// The behaviours.
    async fn behaviours(&self) -> Vec<String> {
        self.relation_type.behaviours.to_vec()
    }

    /// The properties / property types which are defined by the relation type or
    /// by one of the components.
    async fn properties(&self, name: Option<String>) -> Vec<PropertyType> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .relation_type
                .properties
                .to_vec()
                .iter()
                .filter(|property_type| property_type.name == name.clone())
                .cloned()
                .collect();
        }
        self.relation_type.properties.to_vec()
    }

    /// The extensions which are defined by the relation type.
    async fn extensions(&self, name: Option<String>) -> Vec<Extension> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .relation_type
                .extensions
                .to_vec()
                .iter()
                .filter(|extension| extension.name == name.clone())
                .cloned()
                .collect();
        }
        self.relation_type.extensions.to_vec()
    }
}

impl From<RelationType> for GraphQLRelationType {
    fn from(relation_type: RelationType) -> Self {
        GraphQLRelationType { relation_type }
    }
}
