use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::ComponentTypeId;
use crate::RelationTypeId;

/// Addresses the component of a relation type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct RelationComponentTypeId {
    /// The relation type.
    #[builder(setter(into))]
    pub relation_ty: RelationTypeId,

    /// The component type.
    #[builder(setter(into))]
    pub component_ty: ComponentTypeId,
}

impl RelationComponentTypeId {
    pub fn new<R: Into<RelationTypeId>, C: Into<ComponentTypeId>>(relation_ty: R, component_ty: C) -> Self {
        Self { relation_ty: relation_ty.into(), component_ty: component_ty.into() }
    }
}
