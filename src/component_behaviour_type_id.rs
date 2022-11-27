use serde::Deserialize;
use serde::Serialize;

use crate::BehaviourTypeId;
use crate::ComponentTypeId;
use crate::NamespacedType;

/// The behaviour of a component.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentBehaviourTypeId {
    /// The component type.
    pub component_ty: ComponentTypeId,

    /// The behaviour type.
    pub behaviour_ty: BehaviourTypeId,
}

impl ComponentBehaviourTypeId {
    pub fn new(component_ty: ComponentTypeId, behaviour_ty: BehaviourTypeId) -> Self {
        ComponentBehaviourTypeId { component_ty, behaviour_ty }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> Self {
        let namespaced_type = NamespacedType::new(namespace, type_name);
        ComponentBehaviourTypeId::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<NamespacedType> for ComponentBehaviourTypeId {
    fn from(namespaced_type: NamespacedType) -> Self {
        ComponentBehaviourTypeId::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<&BehaviourTypeId> for ComponentBehaviourTypeId {
    fn from(behaviour_ty: &BehaviourTypeId) -> Self {
        ComponentBehaviourTypeId::new(NamespacedType::from(behaviour_ty).into(), behaviour_ty.clone())
    }
}

#[macro_export]
macro_rules! component_behaviour_ty {
    (
        $component_behaviour_type_id: ident,
        $component_type_id: ident,
        $behaviour_type_id: ident
    ) => {
        lazy_static::lazy_static! {
            pub static ref $component_behaviour_type_id: $crate::ComponentBehaviourTypeId = $crate::ComponentBehaviourTypeId::new($component_type_id.clone(), $behaviour_type_id.clone());
        }
    };
}
