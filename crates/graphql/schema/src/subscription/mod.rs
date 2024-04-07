use std::sync::Arc;
use std::time::Duration;

use async_graphql::async_stream;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::Subscription;
use futures_util::Stream;
use futures_util::StreamExt;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

pub use entity_instance::*;
pub use relation_instance::*;

use crate::mutation::GraphQLRelationInstanceId;
use crate::query::GraphQLPropertyInstance;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;

pub mod entity_instance;
pub mod relation_instance;

pub struct InexorSubscription;

/// Subscriptions for the reactive property instances.
#[Subscription(name = "Subscription")]
impl InexorSubscription {
    async fn entity(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "The uuid of the entity instance")] id: Option<Uuid>,
        #[graphql(desc = "The label of the entity instance")] label: Option<String>,
        #[graphql(desc = "The name of the property")] property_name: String,
    ) -> Result<impl Stream<Item = GraphQLPropertyInstance>> {
        match context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>() {
            Ok(entity_instance_manager) => {
                let entity_instance;
                if id.is_some() {
                    entity_instance = entity_instance_manager.get(id.unwrap());
                } else if label.is_some() {
                    entity_instance = entity_instance_manager.get_by_label(label.unwrap().as_str());
                } else {
                    return Err("Either id or label must be given!".into());
                }
                match entity_instance {
                    Some(entity_instance) => {
                        if !entity_instance.properties.contains_key(&property_name) {
                            return Err("Error: property by name not found".into());
                        }
                        let entity_ty = entity_instance.ty.clone();
                        let mut stream = EntityPropertyInstanceStream::new(entity_instance, property_name.clone());

                        Ok(async_stream::stream! {
                            loop {
                                match stream.next().await {
                                    Some(value) => {
                                        futures_timer::Delay::new(Duration::from_millis(10)).await;
                                        yield GraphQLPropertyInstance::new_entity_property(entity_ty.clone(), property_name.clone(), value.clone());
                                    }
                                    None => {
                                        futures_timer::Delay::new(Duration::from_millis(100)).await;
                                    }
                                };
                            }
                        })
                    }
                    None => Err("Error: id not found".into()),
                }
            }
            Err(_) => Err("Error: REIM".into()),
        }
    }

    async fn relation(
        &self,
        context: &Context<'_>,
        edge_key: GraphQLRelationInstanceId,
        #[graphql(desc = "The name of the property")] property_name: String,
    ) -> Result<impl Stream<Item = GraphQLPropertyInstance>> {
        match context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>() {
            Ok(relation_instance_manager) => match relation_instance_manager.get(&edge_key.into()) {
                Some(relation_instance) => {
                    if !relation_instance.properties.contains_key(&property_name) {
                        return Err("Error: property by name not found".into());
                    }
                    let relation_ty = relation_instance.relation_type_id();
                    let mut stream = RelationPropertyInstanceStream::new(relation_instance, property_name.clone());

                    Ok(async_stream::stream! {
                        loop {
                            match stream.next().await {
                                Some(value) => {
                                    futures_timer::Delay::new(Duration::from_millis(10)).await;
                                    yield GraphQLPropertyInstance::new_relation_property(relation_ty.clone(), property_name.clone(), value.clone());
                                }
                                None => {
                                    futures_timer::Delay::new(Duration::from_millis(100)).await;
                                }
                            };
                        }
                    })
                }
                None => Err("Error: id not found".into()),
            },
            Err(_) => Err("Error: REIM".into()),
        }
    }
}

#[derive(Serialize)]
pub struct GraphQLPropertyValueChanged {
    property_name: String,
    value: Value,
}

impl GraphQLPropertyValueChanged {
    pub fn new(property_name: String, value: Value) -> Self {
        GraphQLPropertyValueChanged { property_name, value }
    }
}

impl From<GraphQLPropertyValueChanged> for Value {
    fn from(value_changed: GraphQLPropertyValueChanged) -> Self {
        serde_json::to_value(value_changed).unwrap()
    }
}
