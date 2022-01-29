use std::sync::Arc;

use async_graphql::*;
use uuid::Uuid;

use crate::api::EntityTypeManager;
use crate::graphql::query::{GraphQLEntityInstance, GraphQLEntityType, GraphQLRelationInstance};
use crate::model::ReactiveFlow;

pub struct GraphQLFlow {
    flow: Arc<ReactiveFlow>,
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
#[Object(name = "Flow")]
impl GraphQLFlow {
    /// The id of the flow corresponds to the id of the wrapper entity instance
    ///
    /// This means the vector of entity instances must contain an instance with
    /// the id of the flow.
    async fn id(&self) -> Uuid {
        self.flow.id
    }

    /// The (entity-) type of the flow.
    #[graphql(name = "type")]
    async fn entity_type(&self, context: &Context<'_>) -> Option<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>();
        if entity_type_manager.is_err() {
            return None;
        }
        let entity_type_manager = entity_type_manager.unwrap();
        let entity_type = entity_type_manager.get(self.flow.type_name.clone())?;
        Some(entity_type.into())
    }

    /// The entity instance which is the wrapper for this flow.
    async fn wrapper(&self) -> Option<GraphQLEntityInstance> {
        self.flow.get_wrapper_entity_instance().map(|e| e.into())
    }

    /// The entity instances contained by this flow.
    async fn entities(&self) -> Vec<GraphQLEntityInstance> {
        let reader = self.flow.entity_instances.read().unwrap();
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
        let reader = self.flow.relation_instances.read().unwrap();
        reader
            .iter()
            .map(|(_, relation_instance)| {
                let relation_instance: GraphQLRelationInstance = relation_instance.clone().into();
                relation_instance
            })
            .collect()
    }
}

impl From<Arc<ReactiveFlow>> for GraphQLFlow {
    fn from(flow: Arc<ReactiveFlow>) -> Self {
        GraphQLFlow { flow }
    }
}
