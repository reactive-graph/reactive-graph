use std::sync::Arc;
use std::time::Duration;

use async_graphql::Context;
use async_graphql::Result;
use async_graphql::Subscription;
use async_graphql::async_stream;
use futures_util::Stream;
use futures_util::StreamExt;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

pub use entity_instance::*;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::SubscribeEntityInstanceError;
use reactive_graph_graph::SubscribeRelationInstanceError;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
pub use relation_instance::*;

use crate::mutation::GraphQLRelationInstanceId;
use crate::query::GraphQLPropertyInstance;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;

pub mod entity_instance;
pub mod relation_instance;

pub struct ReactiveGraphSubscription;

/// Subscriptions for the reactive property instances.
#[Subscription(name = "Subscription")]
impl ReactiveGraphSubscription {
    async fn entity(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "The uuid of the entity instance")] id: Option<Uuid>,
        #[graphql(desc = "The label of the entity instance")] label: Option<String>,
        #[graphql(desc = "The name of the property")] property_name: String,
    ) -> Result<impl Stream<Item = GraphQLPropertyInstance>> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let reactive_entity;
        if let Some(id) = id {
            reactive_entity = reactive_entity_manager
                .get(id)
                .ok_or(SubscribeEntityInstanceError::EntityInstanceDoesNotExist(id))?;
        } else if let Some(label) = label {
            reactive_entity = reactive_entity_manager
                .get_by_label(label.as_str())
                .ok_or(SubscribeEntityInstanceError::EntityInstanceWithLabelDoesNotExist(label))?;
        } else {
            return Err(SubscribeEntityInstanceError::EitherUuidOrLabelMustBeGiven.into());
        }
        if !reactive_entity.has_property(&property_name) {
            return Err(SubscribeEntityInstanceError::PropertyNotFound(reactive_entity.id, property_name).into());
        }
        let entity_ty = reactive_entity.ty.clone();
        let mut stream = EntityPropertyInstanceStream::new(reactive_entity, property_name.clone());

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

    async fn relation(
        &self,
        context: &Context<'_>,
        relation_instance_id: GraphQLRelationInstanceId,
        #[graphql(desc = "The name of the property")] property_name: String,
    ) -> Result<impl Stream<Item = GraphQLPropertyInstance>> {
        let reactive_relation_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        let reactive_relation = reactive_relation_manager
            .get(&relation_instance_id)
            .ok_or(SubscribeRelationInstanceError::RelationInstanceDoesNotExist(relation_instance_id))?;
        if !reactive_relation.has_property(&property_name) {
            return Err(SubscribeRelationInstanceError::PropertyNotFound(reactive_relation.id(), property_name).into());
        }
        let relation_ty = reactive_relation.relation_type_id();
        let mut stream = RelationPropertyInstanceStream::new(reactive_relation, property_name.clone());

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
