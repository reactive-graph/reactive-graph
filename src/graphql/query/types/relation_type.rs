use std::sync::Arc;

use async_graphql::*;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::graphql::query::{GraphQLComponent, GraphQLEntityType, GraphQLExtension, GraphQLPropertyType};
use crate::model::RelationType;

pub struct GraphQLRelationType {
    relation_type: RelationType,
}

/// A relation type defines the type of an relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also the relation type defines the properties of the relation instance.
#[Object(name = "RelationType")]
impl GraphQLRelationType {
    /// The outbound entity type(s).
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
            if let Some(entity_type) = entity_type_manager.get(&self.relation_type.outbound_type) {
                return vec![entity_type.into()];
            }
        }
        Vec::new()
    }

    /// The outbound components.
    async fn outbound_components(&self, context: &Context<'_>) -> Vec<GraphQLComponent> {
        match context.data::<Arc<dyn ComponentManager>>() {
            Ok(component_manager) => {
                if let Some(component) = component_manager.get(&self.relation_type.outbound_type) {
                    vec![component.into()]
                } else {
                    Vec::new()
                }
            }
            Err(_) => Vec::new(),
        }
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

    /// The inbound entity type(s).
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
            if let Some(entity_type) = entity_type_manager.get(&self.relation_type.inbound_type) {
                return vec![entity_type.into()];
            }
        }
        Vec::new()
    }

    /// The inbound components.
    async fn inbound_components(&self, context: &Context<'_>) -> Vec<GraphQLComponent> {
        match context.data::<Arc<dyn ComponentManager>>() {
            Ok(component_manager) => {
                if let Some(component) = component_manager.get(&self.relation_type.inbound_type) {
                    vec![component.into()]
                } else {
                    Vec::new()
                }
            }
            Err(_) => Vec::new(),
        }
    }

    /// The namespace the relation type belongs to.
    async fn namespace(&self) -> String {
        self.relation_type.namespace.clone()
    }

    /// Textual description of the relation type.
    async fn description(&self) -> String {
        self.relation_type.description.clone()
    }

    /// The relation type composes it's properties by these components.
    async fn components(&self, context: &Context<'_>) -> Vec<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>();
        if component_manager.is_ok() {
            let component_manager = component_manager.unwrap();
            self.relation_type
                .components
                .iter()
                .filter_map(|component_name| component_manager.get(&component_name))
                .map(|component| component.into())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// The properties / property types which are defined by the relation type or
    /// by one of the components.
    async fn properties(&self, name: Option<String>) -> Vec<GraphQLPropertyType> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .relation_type
                .properties
                .to_vec()
                .iter()
                .filter(|property_type| property_type.name == name.clone())
                .cloned()
                .map(|property_type| property_type.into())
                .collect();
        }
        self.relation_type
            .properties
            .iter()
            .cloned()
            .map(|property_type| property_type.into())
            .collect()
    }

    /// The extensions which are defined by the relation type.
    async fn extensions(&self, name: Option<String>) -> Vec<GraphQLExtension> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .relation_type
                .extensions
                .to_vec()
                .iter()
                .filter(|extension| extension.name == name.clone())
                .cloned()
                .map(|extension| extension.into())
                .collect();
        }
        self.relation_type.extensions.iter().cloned().map(|extension| extension.into()).collect()
    }
}

impl From<RelationType> for GraphQLRelationType {
    fn from(relation_type: RelationType) -> Self {
        GraphQLRelationType { relation_type }
    }
}
