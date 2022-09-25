use std::sync::Arc;

use async_graphql::*;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::RelationTypeManager;
use crate::graphql::query::GraphQLComponent;
use crate::graphql::query::GraphQLEntityType;
use crate::graphql::query::GraphQLFlowType;
use crate::graphql::query::GraphQLRelationType;

#[derive(Default)]
pub struct Types;

/// Search for types (components, entity types or relation types)
#[Object]
impl Types {
    /// Search for components
    ///
    /// Optionally the list of components can be filtered by name.
    async fn components(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters by the name of the components")] name: Option<String>,
        #[graphql(desc = "Searches by the name of the components. Allowed wildcards are: ? and *")] search: Option<String>,
    ) -> Vec<GraphQLComponent> {
        if let Ok(component_manager) = context.data::<Arc<dyn ComponentManager>>() {
            if name.is_some() {
                let component = component_manager.get(&name.unwrap());
                if component.is_some() {
                    return vec![component.unwrap().into()];
                }
                return Vec::new();
            }
            if search.is_some() {
                return component_manager
                    .find(search.unwrap().as_str())
                    .into_iter()
                    .map(|component| component.into())
                    .collect();
            }
            return component_manager.get_components().into_iter().map(|component| component.into()).collect();
        }
        Vec::new()
    }

    async fn count_components(&self, context: &Context<'_>) -> usize {
        context
            .data::<Arc<dyn ComponentManager>>()
            .map(|component_manager| component_manager.count())
            .unwrap_or(0)
    }

    /// Search for entity types.
    ///
    /// Optionally the list of entity types can be filtered by name.
    async fn entities(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters by the name of the entity type")] name: Option<String>,
        #[graphql(desc = "Searches by the name of the entity types. Allowed wildcards are: ? and *")] search: Option<String>,
    ) -> Vec<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>();
        if entity_type_manager.is_ok() {
            let entity_type_manager = entity_type_manager.unwrap();
            if name.is_some() {
                let entity_type = entity_type_manager.get(&name.unwrap());
                if entity_type.is_some() {
                    let entity_type: GraphQLEntityType = entity_type.unwrap().into();
                    return vec![entity_type];
                }
                return Vec::new();
            }
            if search.is_some() {
                return entity_type_manager
                    .find(search.unwrap().as_str())
                    .into_iter()
                    .map(|entity_type| entity_type.into())
                    .collect();
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

    async fn count_entity_types(&self, context: &Context<'_>) -> usize {
        context
            .data::<Arc<dyn EntityTypeManager>>()
            .map(|entity_type_manager| entity_type_manager.count())
            .unwrap_or(0)
    }

    /// Search for relation types.
    async fn relations(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters by outbound entity type")] outbound_type: Option<String>,
        #[graphql(desc = "Filters by the name of the relation type")] name: Option<String>,
        #[graphql(desc = "Searches by the name of the relation types. Allowed wildcards are: ? and *")] search: Option<String>,
        #[graphql(desc = "Filters by inbound entity type")] inbound_type: Option<String>,
    ) -> Vec<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>();
        if relation_type_manager.is_ok() {
            let relation_type_manager = relation_type_manager.unwrap();
            if search.is_some() {
                return relation_type_manager
                    .find(search.unwrap().as_str())
                    .iter()
                    .filter(|relation_type| outbound_type.is_none() || outbound_type.clone().unwrap() == relation_type.outbound_type.clone())
                    .filter(|relation_type| inbound_type.is_none() || inbound_type.clone().unwrap() == relation_type.inbound_type.clone())
                    .map(|relation_type| {
                        let relation_type: GraphQLRelationType = relation_type.clone().into();
                        relation_type
                    })
                    .collect();
            }
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

    async fn count_relation_types(&self, context: &Context<'_>) -> usize {
        context
            .data::<Arc<dyn RelationTypeManager>>()
            .map(|relation_type_manager| relation_type_manager.count())
            .unwrap_or(0)
    }

    /// Search for flow types.
    ///
    /// Optionally the list of flow types can be filtered by name.
    async fn flows(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters by the name of the flow type")] name: Option<String>,
        #[graphql(desc = "Searches by the name of the flow types. Allowed wildcards are: ? and *")] search: Option<String>,
    ) -> Vec<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>();
        if flow_type_manager.is_ok() {
            let flow_type_manager = flow_type_manager.unwrap();
            if name.is_some() {
                let flow_type = flow_type_manager.get(&name.unwrap());
                if flow_type.is_some() {
                    let flow_type: GraphQLFlowType = flow_type.unwrap().into();
                    return vec![flow_type];
                }
                return Vec::new();
            }
            if search.is_some() {
                return flow_type_manager
                    .find(search.unwrap().as_str())
                    .into_iter()
                    .map(|flow_type| flow_type.into())
                    .collect();
            }
            return flow_type_manager
                .get_flow_types()
                .iter()
                .map(|flow_type| {
                    let flow_type: GraphQLFlowType = flow_type.clone().into();
                    flow_type
                })
                .collect();
        }
        Vec::new()
    }

    async fn count_flow_types(&self, context: &Context<'_>) -> usize {
        context
            .data::<Arc<dyn FlowTypeManager>>()
            .map(|flow_type_manager| flow_type_manager.count())
            .unwrap_or(0)
    }
}
