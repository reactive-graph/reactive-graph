use serde::Deserialize;
use serde::Serialize;

use crate::BehaviourTypeId;
use crate::EntityTypeId;
use crate::NamespacedType;

/// The behaviour of an entity type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityBehaviourTypeId {
    /// The entity type.
    pub entity_ty: EntityTypeId,

    /// The behaviour type.
    pub behaviour_ty: BehaviourTypeId,
}

impl EntityBehaviourTypeId {
    pub fn new(entity_ty: EntityTypeId, behaviour_ty: BehaviourTypeId) -> Self {
        EntityBehaviourTypeId { entity_ty, behaviour_ty }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> Self {
        let namespaced_type = NamespacedType::new(namespace, type_name);
        EntityBehaviourTypeId::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<NamespacedType> for EntityBehaviourTypeId {
    fn from(namespaced_type: NamespacedType) -> Self {
        EntityBehaviourTypeId::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}
