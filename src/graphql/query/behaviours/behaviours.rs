use std::sync::Arc;

use async_graphql::*;

use crate::api::EntityBehaviourRegistry;
use crate::api::EntityComponentBehaviourRegistry;
use crate::api::RelationBehaviourRegistry;
use crate::api::RelationComponentBehaviourRegistry;
use crate::graphql::query::GraphQLComponentBehaviour;
use crate::graphql::query::GraphQLEntityBehaviour;
use crate::graphql::query::GraphQLRelationBehaviour;

#[derive(Default)]
pub struct Behaviours;

/// Search for behaviours (component behaviours, entity behaviours or relation behaviours)
#[Object]
impl Behaviours {
    async fn entities(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityBehaviour>> {
        let entity_behaviour_registry = context.data::<Arc<dyn EntityBehaviourRegistry>>()?;
        Ok(entity_behaviour_registry
            .get_all()
            .into_iter()
            .map(|entity_behaviour_ty| entity_behaviour_ty.into())
            .collect())
    }

    async fn count_entity_behaviours(&self, context: &Context<'_>) -> usize {
        match context.data::<Arc<dyn EntityBehaviourRegistry>>() {
            Ok(entity_behaviour_registry) => entity_behaviour_registry.get_all().len(),
            Err(_) => 0,
        }
    }

    async fn entity_components(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponentBehaviour>> {
        let entity_component_behaviour_registry = context.data::<Arc<dyn EntityComponentBehaviourRegistry>>()?;
        Ok(entity_component_behaviour_registry
            .get_all()
            .into_iter()
            .map(|component_behaviour_ty| component_behaviour_ty.into())
            .collect())
    }

    async fn count_entity_component_behaviours(&self, context: &Context<'_>) -> usize {
        match context.data::<Arc<dyn EntityComponentBehaviourRegistry>>() {
            Ok(entity_component_behaviour_registry) => entity_component_behaviour_registry.get_all().len(),
            Err(_) => 0,
        }
    }

    async fn relations(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationBehaviour>> {
        let relation_behaviour_registry = context.data::<Arc<dyn RelationBehaviourRegistry>>()?;
        Ok(relation_behaviour_registry
            .get_all()
            .into_iter()
            .map(|relation_behaviour_ty| relation_behaviour_ty.into())
            .collect())
    }

    async fn count_relation_behaviours(&self, context: &Context<'_>) -> usize {
        match context.data::<Arc<dyn RelationBehaviourRegistry>>() {
            Ok(relation_behaviour_registry) => relation_behaviour_registry.get_all().len(),
            Err(_) => 0,
        }
    }
    async fn relation_components(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponentBehaviour>> {
        let relation_component_behaviour_registry = context.data::<Arc<dyn RelationComponentBehaviourRegistry>>()?;
        Ok(relation_component_behaviour_registry
            .get_all()
            .into_iter()
            .map(|component_behaviour_ty| component_behaviour_ty.into())
            .collect())
    }

    async fn count_relation_component_behaviours(&self, context: &Context<'_>) -> usize {
        match context.data::<Arc<dyn RelationComponentBehaviourRegistry>>() {
            Ok(relation_component_behaviour_registry) => relation_component_behaviour_registry.get_all().len(),
            Err(_) => 0,
        }
    }
}
