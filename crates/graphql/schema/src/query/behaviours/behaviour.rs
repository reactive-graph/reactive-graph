use async_graphql::Object;

use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_graph::NamespacedTypeGetter;

pub struct GraphQLBehaviour {
    behaviour_ty: BehaviourTypeId,
}

/// A behaviour.
#[Object(name = "Behaviour")]
impl GraphQLBehaviour {
    /// The namespace the behaviour type belongs to.
    async fn namespace(&self) -> String {
        self.behaviour_ty.namespace()
    }

    /// The name of the behaviour type.
    async fn name(&self) -> String {
        self.behaviour_ty.type_name()
    }
}

impl From<BehaviourTypeId> for GraphQLBehaviour {
    fn from(behaviour_ty: BehaviourTypeId) -> Self {
        GraphQLBehaviour { behaviour_ty }
    }
}
