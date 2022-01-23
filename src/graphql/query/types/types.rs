use std::sync::Arc;

use async_graphql::*;

use crate::api::{ComponentManager, EntityTypeManager, RelationTypeManager};
use crate::graphql::query::{GraphQLComponent, GraphQLEntityType, GraphQLRelationType};

#[derive(Default)]
pub struct Types;

/// Search for types
#[Object]
impl Types {
    /// Search for components
    ///
    /// Optionally the list of components can be filtered by name.
    async fn components(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters by the name of the components")] name: Option<String>,
    ) -> Vec<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>();
        if component_manager.is_ok() {
            let component_manager = component_manager.unwrap();
            if name.is_some() {
                // TODO: entity_type_manager.search("*query*");
                let component = component_manager.get(name.unwrap());
                if component.is_some() {
                    return vec![component.unwrap().into()];
                }
                return Vec::new();
            }
            return component_manager.get_components().into_iter().map(|component| component.into()).collect();
        }
        Vec::new()
    }

    /// Search for entity types.
    ///
    /// Optionally the list of entity types can be filtered by name.
    async fn entities(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters by the name of the entity type")] name: Option<String>,
    ) -> Vec<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>();
        if entity_type_manager.is_ok() {
            let entity_type_manager = entity_type_manager.unwrap();
            if name.is_some() {
                // TODO: entity_type_manager.search("*query*");
                let entity_type = entity_type_manager.get(name.unwrap());
                if entity_type.is_some() {
                    let entity_type: GraphQLEntityType = entity_type.unwrap().into();
                    return vec![entity_type];
                }
                return Vec::new();
            }
            return entity_type_manager
                .get_entity_types()
                .iter()
                .map(|entity_type| {
                    let entity_type: GraphQLEntityType = entity_type.clone().into();
                    entity_type
                })
                .collect();
        }
        Vec::new()
    }

    /// Search for relation types.
    async fn relations(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters by outbound entity type")] outbound_type: Option<String>,
        #[graphql(desc = "Filters by the name of the relation type")] name: Option<String>,
        #[graphql(desc = "Filters by inbound entity type")] inbound_type: Option<String>,
    ) -> Vec<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>();
        if relation_type_manager.is_ok() {
            let relation_type_manager = relation_type_manager.unwrap();
            return relation_type_manager
                .get_relation_types()
                .iter()
                .filter(|relation_type| outbound_type.is_none() || outbound_type.clone().unwrap() == relation_type.outbound_type.clone())
                .filter(|relation_type| name.is_none() || name.clone().unwrap() == relation_type.type_name.clone())
                .filter(|relation_type| inbound_type.is_none() || inbound_type.clone().unwrap() == relation_type.inbound_type.clone())
                .map(|relation_type| {
                    let relation_type: GraphQLRelationType = relation_type.clone().into();
                    relation_type
                })
                .collect();
        }
        Vec::new()
    }
}
