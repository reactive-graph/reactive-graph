use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use uuid::Uuid;

use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_type_system_api::EntityTypeManager;

use crate::query::GraphQLEntityInstance;
use crate::query::GraphQLEntityType;
use crate::query::GraphQLRelationInstance;

pub struct GraphQLFlowInstance {
    flow_instance: ReactiveFlow,
}

/// A flow is a container for entity instances and relation instances.
///
/// A flow is strictly associated with a wrapper entity instance. The properties
/// of the wrapper entity instance are the properties of the flow.
///
/// Additionally, flows can be nested -  from the perspective of the outer flow
/// the inner flow acts like an entity instance. The wrapper entity instance of
/// the inner flow is the interface which can be accessed by the outer flow.
///
/// Entity instances and relation instances can be shared with multiple flows.
///
/// It's even possible to connect entity instances from different flows with relation
/// instances.
#[Object(name = "FlowInstance")]
impl GraphQLFlowInstance {
    /// The id of the flow corresponds to the id of the wrapper entity instance
    ///
    /// This means the vector of entity instances must contain an instance with
    /// the id of the flow.
    async fn id(&self) -> Uuid {
        self.flow_instance.id
    }

    /// The label of the entity instance if available.
    async fn label(&self) -> Option<String> {
        self.flow_instance
            .get_wrapper_entity_instance()
            .unwrap()
            .properties
            .get("label")
            .and_then(|property_instance| property_instance.as_string())
    }

    /// The (entity-) type of the flow.
    #[graphql(name = "type")]
    async fn entity_type(&self, context: &Context<'_>) -> Option<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>();
        if entity_type_manager.is_err() {
            return None;
        }
        let entity_type_manager = entity_type_manager.unwrap();
        let entity_type = entity_type_manager.get(&self.flow_instance.ty)?;
        Some(entity_type.into())
    }

    /// The entity instance which is the wrapper for this flow.
    async fn wrapper(&self) -> Option<GraphQLEntityInstance> {
        self.flow_instance.get_wrapper_entity_instance().map(|e| e.into())
    }

    /// The entity instances contained by this flow.
    async fn entities(&self) -> Vec<GraphQLEntityInstance> {
        let reader = self.flow_instance.entity_instances.read().unwrap();
        reader
            .iter()
            .map(|(_, entity_instance)| {
                let entity_instance: GraphQLEntityInstance = entity_instance.clone().into();
                entity_instance
            })
            .collect()
    }

    /// The relation instances contained by this flow.
    async fn relations(&self) -> Vec<GraphQLRelationInstance> {
        let reader = self.flow_instance.relation_instances.read().unwrap();
        reader
            .iter()
            .map(|(_, relation_instance)| {
                let relation_instance: GraphQLRelationInstance = relation_instance.clone().into();
                relation_instance
            })
            .collect()
    }
}

impl From<ReactiveFlow> for GraphQLFlowInstance {
    fn from(flow: ReactiveFlow) -> Self {
        GraphQLFlowInstance { flow_instance: flow }
    }
}
