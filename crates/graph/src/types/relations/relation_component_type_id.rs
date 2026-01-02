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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RelationComponentTypeIds(DashSet<RelationComponentTypeId>);

impl RelationComponentTypeIds {
    pub fn new() -> Self {
        Self(DashSet::new())
    }
}

impl Deref for RelationComponentTypeIds {
    type Target = DashSet<RelationComponentTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RelationComponentTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for RelationComponentTypeIds {
    type Item = RelationComponentTypeId;
    type IntoIter = OwningIter<RelationComponentTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<RelationComponentTypeId> for RelationComponentTypeIds {
    fn from_iter<I: IntoIterator<Item = RelationComponentTypeId>>(iter: I) -> Self {
        let tys = RelationComponentTypeIds::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}
