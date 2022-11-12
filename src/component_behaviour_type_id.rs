use serde::Deserialize;
use serde::Serialize;

use crate::BehaviourTypeId;
use crate::ComponentTypeId;

/// The behaviour of a component.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentBehaviourTypeId {
    /// The component type.
    pub component_ty: ComponentTypeId,

    /// The behaviour type.
    pub behaviour_ty: BehaviourTypeId,
}
