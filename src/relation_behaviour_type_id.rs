use serde::Deserialize;
use serde::Serialize;

use crate::BehaviourTypeId;
use crate::NamespacedType;
use crate::RelationTypeId;

/// The behaviour of a relation type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationBehaviourTypeId {
    /// The relation type.
    pub relation_ty: RelationTypeId,

    /// The behaviour type.
    pub behaviour_ty: BehaviourTypeId,
}

impl RelationBehaviourTypeId {
    pub fn new(relation_ty: RelationTypeId, behaviour_ty: BehaviourTypeId) -> Self {
        RelationBehaviourTypeId { relation_ty, behaviour_ty }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> Self {
        let namespaced_type = NamespacedType::new(namespace, type_name);
        RelationBehaviourTypeId::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<NamespacedType> for RelationBehaviourTypeId {
    fn from(namespaced_type: NamespacedType) -> Self {
        RelationBehaviourTypeId::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<&BehaviourTypeId> for RelationBehaviourTypeId {
    fn from(behaviour_ty: &BehaviourTypeId) -> Self {
        RelationBehaviourTypeId::new(NamespacedType::from(behaviour_ty).into(), behaviour_ty.clone())
    }
}
