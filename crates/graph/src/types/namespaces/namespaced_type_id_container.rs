use std::hash::Hash;
use std::ops::Deref;

use dashmap::DashSet;

use crate::Namespace;
use crate::NamespacedType;
use crate::NamespacedTypeConstructor;
use crate::NamespacedTypeIds;
use crate::NamespacedTypeIdsError;

pub trait NamespacedTypeIdContainer
where
    Self: Sized + Deref<Target = DashSet<Self::TypeId>> + Default,
    Self::TypeId: Clone + Eq + PartialEq + Hash + Ord + NamespacedTypeConstructor,
    Self::TypeIds: Clone + FromIterator<Self::TypeId>,
{
    type TypeId;
    type TypeIds;

    fn new() -> Self;

    fn with_namespace<N: Into<Namespace>>(namespace: N) -> Result<NamespacedTypeIds<Self>, NamespacedTypeIdsError>;

    fn parse_namespaces(namespaces: Vec<String>) -> Result<Self, NamespacedTypeIdsError> {
        let tys = Self::new();
        for namespace in namespaces {
            tys.insert(Self::TypeId::new(NamespacedType::try_from(namespace).map_err(NamespacedTypeIdsError::NamespacedTypeError)?));
        }
        Ok(tys)
    }

    fn parse_optional_namespaces(namespaces: Option<Vec<String>>) -> Result<Self, NamespacedTypeIdsError> {
        match namespaces {
            Some(namespaces) => Self::parse_namespaces(namespaces),
            None => Ok(Default::default()),
        }
    }

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
