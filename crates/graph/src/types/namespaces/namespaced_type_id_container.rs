use std::hash::Hash;
use std::ops::Deref;

use crate::NamespacedTypeIds;
use dashmap::DashSet;

pub trait NamespacedTypeIdContainer
where
    Self: Sized + Deref<Target = DashSet<Self::TypeId>>,
    Self::TypeId: Clone + Eq + PartialEq + Hash + Ord,
    Self::TypeIds: Clone + FromIterator<Self::TypeId>,
{
    type TypeId;
    type TypeIds;

    fn new() -> Self;

    fn with_namespace<N: Into<String>>(namespace: N) -> NamespacedTypeIds<Self>;

    fn to_vec(&self) -> Vec<Self::TypeId> {
        let mut tys: Vec<Self::TypeId> = self.iter().map(|ty| ty.clone()).collect();
        tys.sort();
        tys
    }

    fn ty<TY: Into<Self::TypeId>>(self, ty: TY) -> Self {
        self.insert(ty.into());
        self
    }
}
