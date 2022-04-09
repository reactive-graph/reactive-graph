use crate::api::{EntityTypeManager, RelationTypeManager};
use async_graphql::*;
use std::sync::Arc;

use crate::graphql::query::{GraphQLEntityType, GraphQLExtension, GraphQLPropertyType, GraphQLRelationType};
use crate::model::Component;

pub struct GraphQLComponent {
    component: Component,
}

/// Components are composable parts which can be used by types (entity type, relation type).
#[Object(name = "Component")]
impl GraphQLComponent {
    /// The name of the component.
    async fn name(&self) -> String {
        self.component.name.clone()
    }

    /// Textual description of the component.
    async fn description(&self) -> String {
        self.component.description.clone()
    }

    /// The properties which are applied on entity or relation instances.
    async fn properties(&self, name: Option<String>) -> Vec<GraphQLPropertyType> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .component
                .properties
                .to_vec()
                .iter()
                .filter(|property_type| property_type.name == name.clone())
                .cloned()
                .map(|property_type| property_type.into())
                .collect();
        }
        self.component.properties.iter().cloned().map(|property_type| property_type.into()).collect()
    }

    /// The extensions which are defined by the component.
    async fn extensions(&self, name: Option<String>) -> Vec<GraphQLExtension> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .component
                .extensions
                .to_vec()
                .iter()
                .filter(|extension| extension.name == name.clone())
                .cloned()
                .map(|extension| extension.into())
                .collect();
        }
        self.component.extensions.iter().cloned().map(|extension| extension.into()).collect()
    }

    /// Query which entity types are using this component
    async fn entity_types(&self, context: &Context<'_>) -> Vec<GraphQLEntityType> {
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager>>() {
            return entity_type_manager
                .get_entity_types()
                .iter()
                .filter(|entity_type| entity_type.is_a(self.component.name.clone()))
                .cloned()
                .map(|entity_type| entity_type.into())
                .collect();
        }
        Vec::new()
    }

    /// Query which relation types are using this component
    async fn relation_types(&self, context: &Context<'_>) -> Vec<GraphQLRelationType> {
        if let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager>>() {
            return relation_type_manager
                .get_relation_types()
                .iter()
                .filter(|relation_type| relation_type.is_a(self.component.name.clone()))
                .cloned()
                .map(|relation_type| relation_type.into())
                .collect();
        }
        Vec::new()
    }

    /// Query which relation types are using this component as outbound type
    async fn outbound_of(&self, context: &Context<'_>) -> Vec<GraphQLRelationType> {
        if let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager>>() {
            return relation_type_manager
                .get_relation_types()
                .iter()
                .filter(|relation_type| relation_type.outbound_type.eq(&self.component.name))
                .cloned()
                .map(|relation_type| relation_type.into())
                .collect();
        }
        Vec::new()
    }

    /// Query which relation types are using this component as inbound type
    async fn inbound_of(&self, context: &Context<'_>) -> Vec<GraphQLRelationType> {
        if let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager>>() {
            return relation_type_manager
                .get_relation_types()
                .iter()
                .filter(|relation_type| relation_type.inbound_type.eq(&self.component.name))
                .cloned()
                .map(|relation_type| relation_type.into())
                .collect();
        }
        Vec::new()
    }
}

impl From<Component> for GraphQLComponent {
    fn from(component: Component) -> Self {
        GraphQLComponent { component }
    }
}
