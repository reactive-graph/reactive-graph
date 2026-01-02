use crate::Namespace;
use crate::NamespacedType;
use crate::NamespacedTypeConstructor;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypeIds;
use crate::NamespacedTypeIdsError;
use dashmap::DashSet;
use std::hash::Hash;
use std::ops::Deref;
use std::str::FromStr;

#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypes;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypeIds;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeIds;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use rand::prelude::IndexedRandom;
#[cfg(any(test, feature = "test"))]
use rand::rng;

pub trait NamespacedTypeIdContainer
where
    Self: Sized + Deref<Target = DashSet<Self::TypeId>> + Default,
    Self::TypeId: Clone + Eq + PartialEq + Hash + Ord + NamespacedTypeConstructor + NamespacedTypeGetter,
    Self::TypeIds: Clone + FromIterator<Self::TypeId>,
{
    type TypeId;
    type TypeIds;

    fn new() -> Self;

    fn insert(&self, ty: Self::TypeId);

    // fn insert_all(&self, tys: Self::TypeIds) {
    //     let tys = tys;
    //     let iter = Self::TypeIds::into_iter(tys);
    //     for ty in iter {
    //         self.insert(ty);
    //     }
    // }

    fn with_namespace<N: Into<Namespace>>(namespace: N) -> Result<NamespacedTypeIds<Self>, NamespacedTypeIdsError>;

    fn parse_namespaces<I: IntoIterator<Item = NS>, NS: Into<String>>(namespaces: I) -> Result<Self, NamespacedTypeIdsError> {
        let tys = Self::new();
        for namespace in namespaces {
            let namespace = namespace.into();
            tys.insert(Self::TypeId::new(NamespacedType::from_str(&namespace).map_err(NamespacedTypeIdsError::NamespacedTypeError)?));
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

    fn into_fully_qualified_namespaces(self) -> Vec<String> {
        self.iter().map(|ty| ty.namespace().to_string()).collect()
    }

    #[cfg(any(test, feature = "test"))]
    fn pick_random_type_id(&self) -> Option<Self::TypeId> {
        self.to_vec().choose(&mut rng()).map(Clone::clone)
    }

    #[cfg(any(test, feature = "test"))]
    fn pick_random_type_ids(&self, max_types: usize) -> Self {
        let mut rng = rand::rng();
        let type_ids = self.to_vec();
        let random_type_ids = Self::new();
        for _ in 0..rng.random_range(0..max_types) {
            if let Some(random_type_id) = type_ids.choose(&mut rng) {
                random_type_ids.insert(random_type_id.clone());
            }
        }
        random_type_ids
    }
}

#[cfg(any(test, feature = "test"))]
impl<TY: RandomNamespacedTypeId<Error = NamespacedTypeError>, TYS: NamespacedTypeIdContainer<TypeIds = TYS, TypeId = TY>> RandomNamespacedTypeIds for TYS {
    type Error = NamespacedTypeError;

    fn random_type_ids() -> Result<Self, NamespacedTypeError> {
        let tys = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            let ty = TY::random_type_id()?;
            tys.insert(ty);
        }
        Ok(tys)
    }
}

#[cfg(any(test, feature = "test"))]
impl<TY: RandomNamespacedTypeId<Error = NamespacedTypeError> + NamespacedTypeConstructor, TYS: NamespacedTypeIdContainer<TypeIds = TYS, TypeId = TY>>
    RandomChildTypeIds for TYS
{
    type Error = NamespacedTypeError;
    fn random_child_type_ids(namespace: &Namespace) -> Result<Self, Self::Error> {
        let tys = Self::new();
        for random_child_ty in NamespacedTypes::random_child_type_ids(namespace)? {
            tys.insert(NamespacedTypeConstructor::new(random_child_ty));
        }
        Ok(tys)
    }
}
