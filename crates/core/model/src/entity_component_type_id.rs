use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::ComponentTypeId;
use crate::EntityTypeId;

/// Addresses the component of an entity type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct EntityComponentTypeId {
    /// The entity type.
    pub entity_ty: EntityTypeId,

    /// The component type.
    pub component_ty: ComponentTypeId,
}

impl EntityComponentTypeId {
    pub fn new(entity_ty: EntityTypeId, component_ty: ComponentTypeId) -> Self {
        EntityComponentTypeId { entity_ty, component_ty }
    }
}
