use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use reactive_graph_graph::InvalidFlowInstanceError;
use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_type_system_api::EntityTypeManager;
use uuid::Uuid;

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

    /// The name of the flow instance.
    async fn name(&self) -> String {
        self.flow_instance.name.clone()
    }

    /// Textual description of the flow instance.
    async fn description(&self) -> String {
        self.flow_instance.description.clone()
    }

    /// The (entity-) type of the flow.
    #[graphql(name = "type")]
    async fn entity_type(&self, context: &Context<'_>) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        entity_type_manager
            .get(&self.flow_instance.ty)
            .map(Into::into)
            .ok_or(InvalidFlowInstanceError::EntityTypeDoesNotExist(self.flow_instance.ty.clone()).into())
    }

    /// The entity instance which is the wrapper for this flow.
    async fn wrapper(&self) -> Option<GraphQLEntityInstance> {
        self.flow_instance.get_wrapper_entity_instance().map(|e| e.into())
    }

    /// The entity instances contained by this flow.
    async fn entities(&self) -> Vec<GraphQLEntityInstance> {
        let reader = self.flow_instance.entity_instances.read().unwrap();
        reader
            .values()
            .map(|reactive_entity| {
                let entity_instance: GraphQLEntityInstance = reactive_entity.clone().into();
                entity_instance
            })
            .collect()
    }

    /// The relation instances contained by this flow.
    async fn relations(&self) -> Vec<GraphQLRelationInstance> {
        let reader = self.flow_instance.relation_instances.read().unwrap();
        reader
            .values()
            .map(|reactive_relation| {
                let relation_instance: GraphQLRelationInstance = reactive_relation.clone().into();
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
