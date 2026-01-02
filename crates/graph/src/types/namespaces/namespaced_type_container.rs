use std::hash::Hash;
use std::ops::Deref;

use dashmap::DashMap;
use dashmap::DashSet;
use wildmatch::WildMatch;

use crate::Namespace;
use crate::NamespacedTypeGetter;
use crate::Namespaces;

#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeIdContainer;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildType;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypes;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedType;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypes;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypesWithId;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use rand::rng;
#[cfg(any(test, feature = "test"))]
use rand::seq::IndexedRandom;
#[cfg(any(test, feature = "test"))]
use std::ops::Range;

pub trait NamespacedTypeContainer
where
    Self: Sized + Deref<Target = DashMap<Self::TypeId, Self::Type>> + FromIterator<Self::Type> + From<DashMap<Self::TypeId, Self::Type>>,
    Self::TypeId: Clone + Eq + PartialEq + Hash,
    Self::TypeIds: Clone + FromIterator<Self::TypeId>,
    Self::Type: Clone + Hash + Ord + Sized + NamespacedTypeGetter + AsRef<Self::TypeId>,
{
    type TypeId;
    type TypeIds;
    type Type;

    fn new() -> Self {
        DashMap::<Self::TypeId, Self::Type>::new().into()
    }

    fn push<T: Into<Self::Type>>(&self, type_: T) -> Option<Self::Type> {
        let type_ = type_.into();
        DashMap::<Self::TypeId, Self::Type>::insert(self.deref(), type_.as_ref().clone(), type_)
    }

    fn type_ids(&self) -> Self::TypeIds {
        self.iter().map(|item| item.key().clone()).collect()
    }

    fn types(&self) -> DashSet<Self::Type> {
        self.iter().map(|item| item.value().clone()).collect()
    }

    fn to_vec(&self) -> Vec<Self::Type> {
        let mut items: Vec<_> = self.iter().map(|item| item.value().clone()).collect();
        items.sort();
        items
    }

    fn namespaces(&self) -> Namespaces {
        self.iter().map(|item| item.path()).collect()
    }

    fn get_by_namespace<N: Into<Namespace>>(&self, namespace: N) -> Self {
        let namespace = namespace.into();
        self.iter().filter(|item| item.path() == namespace).map(|item| item.value().clone()).collect()
    }

    fn get_types_by_namespace<N: Into<Namespace>>(&self, namespace: N) -> Self::TypeIds {
        let namespace = namespace.into();
        self.iter().filter(|item| item.path() == namespace).map(|item| item.key().clone()).collect()
    }

    fn find(&self, search: &str) -> Self {
        let matcher = WildMatch::new(search);
        self.iter()
            .filter(|item| matcher.matches(item.namespace().to_string().as_ref()) || matcher.matches(item.type_name().as_ref()))
            .map(|item| item.value().clone())
            .collect()
    }

    fn count_by_namespace<N: Into<Namespace>>(&self, namespace: N) -> usize {
        let namespace = namespace.into();
        self.iter().filter(|item| item.path() == namespace).count()
    }

    fn push_all(&self, types: Self) {
        for type_ in types.deref().into_iter() {
            self.insert(type_.key().clone(), type_.value().clone());
        }
    }

    #[cfg(any(test, feature = "test"))]
    fn pick_random_type(&self) -> Option<Self::Type> {
        self.to_vec().choose(&mut rng()).map(Clone::clone)
    }

    #[cfg(any(test, feature = "test"))]
    fn pick_random_types(&self, range: Range<usize>) -> Self {
        let mut rng = rand::rng();
        let types = self.to_vec();
        let random_types = Self::new();
        for _ in 0..rng.random_range(range) {
            if let Some(random_type) = types.choose(&mut rng) {
                random_types.push(random_type.clone());
            }
        }
        random_types
    }
}

#[cfg(any(test, feature = "test"))]
impl<
    TY: Eq + Hash,
    TYS: NamespacedTypeIdContainer<TypeIds = TYS, TypeId = TY>,
    T: RandomNamespacedType<Error = NamespacedTypeError>,
    TS: NamespacedTypeContainer<TypeIds = TYS, TypeId = TY, Type = T> + Sized,
> RandomNamespacedTypes for TS
{
    type Error = NamespacedTypeError;

    fn random_types(range: Range<usize>) -> Result<Self, NamespacedTypeError> {
        let tys = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(range) {
            let ty = T::random_type()?;
            tys.push(ty);
        }
        Ok(tys)
    }
}

#[cfg(any(test, feature = "test"))]
impl<
    TY: Eq + Hash,
    TYS: NamespacedTypeIdContainer<TypeIds = TYS, TypeId = TY>,
    T: RandomNamespacedType<Error = NamespacedTypeError, TypeId = TY>,
    TS: NamespacedTypeContainer<TypeIds = TYS, TypeId = TY, Type = T> + Sized,
> RandomNamespacedTypesWithId for TS
{
    type TypeIds = TYS;

    fn random_types_with_ids(tys: &TYS) -> Result<Self, NamespacedTypeError> {
        let types = Self::new();
        for ty in tys.deref().iter() {
            let type_ = T::random_type_with_id(ty.key())?;
            types.push(type_);
        }
        Ok(types)
    }
}

#[cfg(any(test, feature = "test"))]
impl<
    TY,
    TYS: NamespacedTypeIdContainer<TypeIds = TYS, TypeId = TY>,
    T: RandomChildType<Error = NamespacedTypeError>,
    TS: NamespacedTypeContainer<TypeIds = TYS, TypeId = TY, Type = T> + Sized,
> RandomChildTypes for TS
{
    type Error = NamespacedTypeError;

    fn random_child_types(namespace: &Namespace, range: Range<usize>) -> Result<Self, NamespacedTypeError> {
        let tys = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(range) {
            let ty = T::random_child_type(namespace)?;
            tys.push(ty);
        }
        Ok(tys)
    }
}
