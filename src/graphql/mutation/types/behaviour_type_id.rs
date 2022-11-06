use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;

use crate::model::BehaviourTypeId;

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
