use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use typed_builder::TypedBuilder;

use crate::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedType;

/// The behaviour of an entity type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
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

    pub fn new_from_type<N: Into<String>, T: Into<String>>(namespace: N, type_name: T) -> Self {
        let namespaced_type = NamespacedType::new(namespace, type_name);
        EntityBehaviourTypeId::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<NamespacedType> for EntityBehaviourTypeId {
    fn from(namespaced_type: NamespacedType) -> Self {
        EntityBehaviourTypeId::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<&BehaviourTypeId> for EntityBehaviourTypeId {
    fn from(behaviour_ty: &BehaviourTypeId) -> Self {
        EntityBehaviourTypeId::new(NamespacedType::from(behaviour_ty).into(), behaviour_ty.clone())
    }
}

impl Display for EntityBehaviourTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", &self.entity_ty, &self.behaviour_ty)
    }
}

#[macro_export]
macro_rules! entity_behaviour_ty {
    (
        $entity_behaviour_type_id: ident,
        $entity_type_id: ident,
        $behaviour_type_id: ident
    ) => {
        lazy_static::lazy_static! {
            pub static ref $entity_behaviour_type_id: $crate::EntityBehaviourTypeId = $crate::EntityBehaviourTypeId::new($entity_type_id.clone(), $behaviour_type_id.clone());
        }
    };
}
