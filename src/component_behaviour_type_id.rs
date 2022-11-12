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
