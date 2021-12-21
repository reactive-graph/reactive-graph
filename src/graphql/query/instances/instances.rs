use std::sync::Arc;

use async_graphql::*;
use uuid::Uuid;

use crate::api::{ReactiveEntityInstanceManager, ReactiveRelationInstanceManager};
use crate::graphql::query::{GraphQLEntityInstance, GraphQLRelationInstance};

#[derive(Default)]
pub struct Instances;

/// Search for instances
#[Object]
impl Instances {
    /// Search for entity instances.
    ///
    /// If an id is given, the entity instance with the given id will be returned.
    ///
    /// If an entity type is given, only entity instances of the given type are returned.
    async fn entities(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Returns only the entity instance with the given id.")] id: Option<Uuid>,
        #[graphql(desc = "Filters the entity instances by type.")] entity_type: Option<String>,
    ) -> Vec<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>();
        if entity_instance_manager.is_ok() {
            let entity_instance_manager = entity_instance_manager.unwrap();
            if id.is_some() {
                let entity_instance = entity_instance_manager.get(id.unwrap()).map(|entity_instance| {
                    let entity_instance: GraphQLEntityInstance = entity_instance.clone().into();
                    entity_instance
                });
                return if entity_instance.is_some() {
                    vec![entity_instance.unwrap()]
                } else {
                    Vec::new()
                };
            }
            return entity_instance_manager
                .get_entity_instances()
                .iter()
                .filter(|entity_instance| entity_type.is_none() || entity_type.clone().unwrap() == entity_instance.type_name.clone())
                .map(|entity_instance| {
                    let entity_instance: GraphQLEntityInstance = entity_instance.clone().into();
                    entity_instance
                })
                .collect();
        }
        Vec::new()
    }

    async fn relations(
        &self,
        context: &Context<'_>,
        outbound_type: Option<String>,
        #[graphql(desc = "Filters the relation instances by relation type")] relation_type: Option<String>,
        inbound_type: Option<String>,
    ) -> Vec<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>();
        if relation_instance_manager.is_ok() {
            let relation_instance_manager = relation_instance_manager.unwrap();
            return relation_instance_manager
                .get_relation_instances()
                .iter()
                .filter(|relation_instance| relation_type.is_none() || relation_type.clone().unwrap() == relation_instance.type_name.clone())
                .filter(|relation_instance| {
                    // TODO: handle starts with?
                    outbound_type.is_none() || outbound_type.clone().unwrap() == relation_instance.outbound.clone().type_name.clone()
                })
                .filter(|relation_instance| {
                    // TODO: handle starts with?
                    inbound_type.is_none() || inbound_type.clone().unwrap() == relation_instance.inbound.clone().type_name.clone()
                })
                .map(|relation_instance| {
                    let relation_instance: GraphQLRelationInstance = relation_instance.clone().into();
                    relation_instance
                })
                .collect();
        }
        Vec::new()
    }
}
