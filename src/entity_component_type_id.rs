use serde::Deserialize;
use serde::Serialize;

use crate::ComponentTypeId;
use crate::EntityTypeId;

/// Addresses the component of an entity type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityComponentTypeId {
    /// The entity type.
    pub entity_ty: EntityTypeId,

    /// The component type.
    pub component_ty: ComponentTypeId,
}
