use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::ComponentContainerGetter;
use crate::ComponentTypeId;
use crate::EntityTypeId;
use crate::NamespacedType;

/// Addresses the component of an entity type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct EntityComponentTypeId {
    /// The entity type.
    #[builder(setter(into))]
    pub entity_ty: EntityTypeId,

    /// The component type.
    #[builder(setter(into))]
    pub component_ty: ComponentTypeId,
}

impl EntityComponentTypeId {
    pub fn new<E: Into<EntityTypeId>, C: Into<ComponentTypeId>>(entity_ty: E, component_ty: C) -> Self {
        Self {
            entity_ty: entity_ty.into(),
            component_ty: component_ty.into(),
        }
    }
}

impl ComponentContainerGetter for EntityComponentTypeId {
    fn container_ty(&self) -> NamespacedType {
        NamespacedType::from(&self.entity_ty)
    }

    fn component_ty(&self) -> ComponentTypeId {
        self.component_ty.clone()
    }
}
