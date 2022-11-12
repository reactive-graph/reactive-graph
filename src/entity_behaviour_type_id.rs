use serde::Deserialize;
use serde::Serialize;

use crate::BehaviourTypeId;
use crate::EntityTypeId;

/// The behaviour of an entity type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityBehaviourTypeId {
    /// The entity type.
    pub entity_ty: EntityTypeId,

    /// The behaviour type.
    pub behaviour_ty: BehaviourTypeId,
}
