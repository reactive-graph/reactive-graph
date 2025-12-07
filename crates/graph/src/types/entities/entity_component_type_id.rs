use dashmap::DashSet;
use dashmap::iter_set::OwningIter;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::hash::RandomState;
use std::ops::Deref;
use std::ops::DerefMut;
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EntityComponentTypeIds(DashSet<EntityComponentTypeId>);

impl EntityComponentTypeIds {
    pub fn new() -> Self {
        Self(DashSet::new())
    }
}

impl Deref for EntityComponentTypeIds {
    type Target = DashSet<EntityComponentTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityComponentTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for EntityComponentTypeIds {
    type Item = EntityComponentTypeId;
    type IntoIter = OwningIter<EntityComponentTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<EntityComponentTypeId> for EntityComponentTypeIds {
    fn from_iter<I: IntoIterator<Item = EntityComponentTypeId>>(iter: I) -> Self {
        let tys = EntityComponentTypeIds::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}
