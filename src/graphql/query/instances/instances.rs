use std::sync::Arc;

use async_graphql::*;
use inexor_rgf_core_model::ReactivePropertyInstance;
use uuid::Uuid;

use crate::api::{ReactiveEntityInstanceManager, ReactiveRelationInstanceManager};
use crate::graphql::query::{GraphQLEntityInstance, GraphQLPropertyInstance, GraphQLRelationInstance};

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
        #[graphql(desc = "Returns the entity instance with the given label.")] label: Option<String>,
        #[graphql(name = "type", desc = "Filters the entity instances by type.")] entity_type: Option<String>,
        #[graphql(name = "properties", desc = "Query by properties.")] property_query: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Vec<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>();
        if entity_instance_manager.is_ok() {
            let entity_instance_manager = entity_instance_manager.unwrap();
            if id.is_some() {
                let entity_instance = entity_instance_manager.get(id.unwrap()).map(|entity_instance| {
                    let entity_instance: GraphQLEntityInstance = entity_instance.into();
                    entity_instance
                });
                return if entity_instance.is_some() {
                    vec![entity_instance.unwrap()]
                } else {
                    Vec::new()
                };
            }
            if label.is_some() {
                let entity_instance = entity_instance_manager.get_by_label(label.unwrap()).map(|entity_instance| {
                    let entity_instance: GraphQLEntityInstance = entity_instance.into();
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
                .filter(|entity_instance| {
                    property_query.is_none() || {
                        let property_query = property_query.clone().unwrap();
                        if property_query.is_empty() {
                            return true;
                        }
                        if entity_instance.properties.is_empty() {
                            return false;
                        }
                        property_query
                            .iter()
                            .all(|property_query| match entity_instance.properties.get(property_query.name.as_str()) {
                                Some(property_instance) => property_query.value == property_instance.get(),
                                None => false,
                            })
                    }
                })
                .map(|entity_instance| {
                    let entity_instance: GraphQLEntityInstance = entity_instance.clone().into();
                    entity_instance
                })
                .collect();
        }
        Vec::new()
    }

    /// Search for relations instances.
    ///
    /// Relation instances can be searched by relation type name, the entity type of the outbound
    /// entity instance, the entity type of the inbound entity instance, the id of the outbound
    /// entity instance or the id of the inbound entity instance. All of these filters can be
    /// combined.
    async fn relations(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters the relation instances by the entity type of the outbound entity instance")] outbound_type: Option<String>,
        #[graphql(name = "type", desc = "Filters the relation instances by relation type")] relation_type: Option<String>,
        #[graphql(desc = "Filters the relation instances by the entity type of the inbound entity instance")] inbound_type: Option<String>,
        #[graphql(desc = "Filters the relation instances by the id of the outbound entity instance")] outbound_id: Option<Uuid>,
        #[graphql(desc = "Filters the relation instances by the id of the inbound entity instance")] inbound_id: Option<Uuid>,
        #[graphql(name = "properties", desc = "Query by properties.")] property_query: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Vec<GraphQLRelationInstance> {
        if let Ok(relation_instance_manager) = context.data::<Arc<dyn ReactiveRelationInstanceManager>>() {
            return relation_instance_manager
                .get_relation_instances()
                .iter()
                .filter(|relation_instance| match &relation_type {
                    Some(relation_type) => relation_instance.type_name.starts_with(relation_type),
                    None => true,
                })
                .filter(|relation_instance| outbound_type.is_none() || outbound_type.clone().unwrap() == relation_instance.outbound.clone().type_name.clone())
                .filter(|relation_instance| inbound_type.is_none() || inbound_type.clone().unwrap() == relation_instance.inbound.clone().type_name.clone())
                .filter(|relation_instance| outbound_id.is_none() || outbound_id.unwrap() == relation_instance.outbound.id)
                .filter(|relation_instance| inbound_id.is_none() || inbound_id.unwrap() == relation_instance.inbound.id)
                .filter(|relation_instance| {
                    property_query.is_none() || {
                        let property_query = property_query.clone().unwrap();
                        if property_query.is_empty() {
                            return true;
                        }
                        if relation_instance.properties.is_empty() {
                            return false;
                        }
                        property_query
                            .iter()
                            .all(|property_query| match relation_instance.properties.get(property_query.name.as_str()) {
                                Some(property_instance) => property_query.value == property_instance.get(),
                                None => false,
                            })
                    }
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
