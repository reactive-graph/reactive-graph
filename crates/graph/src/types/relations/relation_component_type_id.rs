use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::ComponentContainerGetter;
use crate::ComponentTypeId;
use crate::NamespacedType;
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
        Self {
            relation_ty: relation_ty.into(),
            component_ty: component_ty.into(),
        }
    }
}

impl ComponentContainerGetter for RelationComponentTypeId {
    fn container_ty(&self) -> NamespacedType {
        NamespacedType::from(&self.relation_ty)
    }

    fn component_ty(&self) -> ComponentTypeId {
        self.component_ty.clone()
    }
}
