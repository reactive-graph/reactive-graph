use serde::Deserialize;
use serde::Serialize;

use crate::ComponentTypeId;
use crate::RelationTypeId;

/// Addresses the component of a relation type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationComponentTypeId {
    /// The relation type.
    pub relation_ty: RelationTypeId,

    /// The component type.
    pub component_ty: ComponentTypeId,
}

impl RelationComponentTypeId {
    pub fn new(relation_ty: RelationTypeId, component_ty: ComponentTypeId) -> Self {
        RelationComponentTypeId { relation_ty, component_ty }
    }
}
