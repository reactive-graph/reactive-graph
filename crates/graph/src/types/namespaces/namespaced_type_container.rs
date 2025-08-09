use std::hash::Hash;
use std::ops::Deref;

use dashmap::DashMap;
use dashmap::DashSet;
use wildmatch::WildMatch;

use crate::Namespace;
use crate::NamespacedTypeGetter;
use crate::Namespaces;

pub trait NamespacedTypeContainer
where
    Self: Sized + Deref<Target = DashMap<Self::TypeId, Self::Type>> + FromIterator<Self::Type>,
    Self::TypeId: Clone + Eq + PartialEq + Hash,
    Self::TypeIds: Clone + FromIterator<Self::TypeId>,
    Self::Type: Clone + Hash + Ord + Sized + NamespacedTypeGetter,
{
    type TypeId;
    type TypeIds;
    type Type;

    fn new() -> Self;

    fn push<I: Into<Self::Type>>(&self, item_to_add: I);

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
}
