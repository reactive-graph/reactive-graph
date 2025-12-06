use async_graphql::Object;

use crate::query::GraphQLNamespacedType;
use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_graph::NamespacedTypeGetter;

pub struct GraphQLBehaviour {
    behaviour_ty: BehaviourTypeId,
}

/// A behaviour.
#[Object(name = "Behaviour")]
impl GraphQLBehaviour {
    /// The fully qualified namespace of the behaviour.
    #[graphql(name = "type")]
    async fn ty(&self) -> String {
        self.behaviour_ty.namespace().to_string()
    }

    /// The namespaced type.
    async fn namespaced_type(&self) -> GraphQLNamespacedType {
        self.behaviour_ty.namespaced_type().into()
    }
}

impl From<BehaviourTypeId> for GraphQLBehaviour {
    fn from(behaviour_ty: BehaviourTypeId) -> Self {
        GraphQLBehaviour { behaviour_ty }
    }
}
