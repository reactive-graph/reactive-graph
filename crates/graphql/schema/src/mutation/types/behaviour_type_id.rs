use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use serde::Deserialize;
use serde::Serialize;

use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_graph::NamespacedTypeGetter;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "BehaviourTypeId")]
pub struct BehaviourTypeIdDefinition {
    /// The namespace of the behaviour type.
    pub namespace: String,

    /// The name of the behaviour type.
    #[graphql(name = "name")]
    pub type_name: String,
}

impl From<BehaviourTypeIdDefinition> for BehaviourTypeId {
    fn from(ty: BehaviourTypeIdDefinition) -> Self {
        BehaviourTypeId::new_from_type(ty.namespace, ty.type_name)
    }
}

impl From<BehaviourTypeId> for BehaviourTypeIdDefinition {
    fn from(ty: BehaviourTypeId) -> Self {
        BehaviourTypeIdDefinition {
            namespace: ty.namespace(),
            type_name: ty.type_name(),
        }
    }
}
