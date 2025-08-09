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
    /// The namespace and type name.
    #[graphql(name = "type")]
    async fn ty(&self) -> GraphQLNamespacedType {
        self.behaviour_ty.namespaced_type().into()
    }
}

impl From<BehaviourTypeId> for GraphQLBehaviour {
    fn from(behaviour_ty: BehaviourTypeId) -> Self {
        GraphQLBehaviour { behaviour_ty }
    }
}
