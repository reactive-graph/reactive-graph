use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_behaviour_service_api::EntityBehaviourRegistry;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourRegistry;
use reactive_graph_behaviour_service_api::RelationBehaviourRegistry;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourRegistry;

use crate::query::GraphQLComponentBehaviour;
use crate::query::GraphQLEntityBehaviour;
use crate::query::GraphQLRelationBehaviour;

#[derive(Default)]
pub struct Behaviours;

/// Search for behaviours (component behaviours, entity behaviours or relation behaviours)
#[Object]
impl Behaviours {
    async fn entities(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityBehaviour>> {
        let entity_behaviour_registry = context.data::<Arc<dyn EntityBehaviourRegistry + Send + Sync>>()?;
        Ok(entity_behaviour_registry
            .get_all()
            .into_iter()
            .map(|entity_behaviour_ty| entity_behaviour_ty.into())
            .collect())
    }

    async fn count_entity_behaviours(&self, context: &Context<'_>) -> usize {
        match context.data::<Arc<dyn EntityBehaviourRegistry + Send + Sync>>() {
            Ok(entity_behaviour_registry) => entity_behaviour_registry.get_all().len(),
            Err(_) => 0,
        }
    }

    async fn entity_components(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponentBehaviour>> {
        let entity_component_behaviour_registry = context.data::<Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>>()?;
        Ok(entity_component_behaviour_registry
            .get_all()
            .into_iter()
            .map(|component_behaviour_ty| component_behaviour_ty.into())
            .collect())
    }

    async fn count_entity_component_behaviours(&self, context: &Context<'_>) -> usize {
        match context.data::<Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>>() {
            Ok(entity_component_behaviour_registry) => entity_component_behaviour_registry.get_all().len(),
            Err(_) => 0,
        }
    }

    async fn relations(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationBehaviour>> {
        let relation_behaviour_registry = context.data::<Arc<dyn RelationBehaviourRegistry + Send + Sync>>()?;
        Ok(relation_behaviour_registry
            .get_all()
            .into_iter()
            .map(|relation_behaviour_ty| relation_behaviour_ty.into())
            .collect())
    }

    async fn count_relation_behaviours(&self, context: &Context<'_>) -> usize {
        match context.data::<Arc<dyn RelationBehaviourRegistry + Send + Sync>>() {
            Ok(relation_behaviour_registry) => relation_behaviour_registry.get_all().len(),
            Err(_) => 0,
        }
    }
    async fn relation_components(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponentBehaviour>> {
        let relation_component_behaviour_registry = context.data::<Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>>()?;
        Ok(relation_component_behaviour_registry
            .get_all()
            .into_iter()
            .map(|component_behaviour_ty| component_behaviour_ty.into())
            .collect())
    }

    async fn count_relation_component_behaviours(&self, context: &Context<'_>) -> usize {
        match context.data::<Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>>() {
            Ok(relation_component_behaviour_registry) => relation_component_behaviour_registry.get_all().len(),
            Err(_) => 0,
        }
    }
}
