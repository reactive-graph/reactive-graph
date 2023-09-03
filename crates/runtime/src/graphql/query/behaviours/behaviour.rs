use async_graphql::*;

use crate::reactive::BehaviourTypeId;
use crate::model::NamespacedTypeGetter;

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
