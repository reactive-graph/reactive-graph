use serde::Deserialize;
use serde::Serialize;

use crate::BehaviourTypeId;
use crate::RelationTypeId;

/// The behaviour of a relation type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationBehaviourTypeId {
    /// The relation type.
    pub relation_ty: RelationTypeId,

    /// The behaviour type.
    pub behaviour_ty: BehaviourTypeId,
}
