use std::hash::Hash;
use std::ops::Deref;

use dashmap::{DashMap, DashSet};
use wildmatch::WildMatch;

use crate::NamespacedTypeGetter;
use crate::types::Namespaces;

pub trait NamespacedTypeContainer
    where
        Self: Sized + Deref<Target=DashMap<Self::TypeId, Self::Type>> + FromIterator<Self::Type>,
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
        self.iter()
            .map(|item| item.key().clone())
            .collect()
    }

    fn types(&self) -> DashSet<Self::Type> {
        self.iter()
            .map(|item| item.value().clone())
            .collect()
    }

    fn to_vec(&self) -> Vec<Self::Type> {
        let mut items: Vec<_> = self.iter()
            .map(|item| item.value().clone())
            .collect();
        items.sort();
        items
    }

    fn namespaces(&self) -> Namespaces {
        self.iter()
            .map(|item| item.namespace())
            .collect()
    }

    fn get_by_namespace(&self, namespace: &str) -> Self {
        self.iter()
            .filter(|item| item.namespace() == namespace)
            .map(|item| item.value().clone())
            .collect()
    }

    fn get_types_by_namespace(&self, namespace: &str) -> Self::TypeIds {
        self.iter()
            .filter(|item| item.namespace() == namespace)
            .map(|item| item.key().clone())
            .collect()
    }

    fn find_by_type_name(&self, search: &str) -> Self {
        let matcher = WildMatch::new(search);
        self.iter()
            .filter(|item| matcher.matches(item.type_name().as_str()))
            .map(|item| item.value().clone())
            .collect()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.iter()
            .filter(|item| item.namespace() == namespace)
            .count()
    }


}

// impl<O, I> From<I> for O
//     where
//         O: Default + Grid,
//         O::Component: Copy,
//         I: Grid<Component = O::Component, WIDTH = O::WIDTH, HEIGHT = O::HEIGHT>,
// {}
//     <ID=ID, Item=Item, Target=DashMap<ID, Item>





// impl <ID, Item> FromIterator<Item> for dyn NamespacedTypeContainer<ID=ID, Item=Item, Target=DashMap<ID, Item>>
// // where
//     //     Self: Deref<Target=DashMap<Self::ID, Self::Type>> + FromIterator<Self::Type>,
//     //     Self::ID: Eq + PartialEq + Hash,
//     //     Self::Type: Clone + Ord + NamespacedTypeGetter,
// {
//     fn from_iter<I: IntoIterator<Item=Item>>(iter: I) -> Self {
//         let entity_types = Self::new();
//         for entity_type in iter {
//             entity_types.insert(entity_type.ty.clone(), entity_type);
//         }
//         entity_types
//     }
// }
//
// impl PartialEq for dyn NamespacedTypeContainer {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.iter().all(|self_entity_type| other.contains_key(&self_entity_type.ty))
//             && other.iter().all(|other_entity_type| self.contains_key(&other_entity_type.ty))
//     }
// }
//
// impl IntoIterator for NamespacedTypeContainer {
//     type Item = (EntityTypeId, EntityType);
//     type IntoIter = OwningIter<EntityTypeId, EntityType>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl <ID, Item> Deref for dyn NamespacedTypeContainer<ID=ID, Item=Item, Target=DashMap<ID, Item>> {
//     type Target = DashMap<ID, Item>;
//     // type Target = DashMap<NamespacedTypeContainer::ID, NamespacedTypeContainer::Item>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl DerefMut for NamespacedTypeContainer {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl<R: RngCore + ?Sized, T: DerefMut<Target = R>> RngCore for T { ... }

// impl <ID, Item> Hash for dyn NamespacedTypeContainer<ID, Item, Target=DashMap<ID, Item>>
// impl <ID, Item> Hash for dyn NamespacedTypeContainer<ID=ID, Item=Item, Target=DashMap<ID, Item>> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // let mut items: Vec<Self::Type> = self.iter()
//         //     .map(|item| item.value().clone()).collect();
//         // items.sort();
//         // items.hash(state);
//
//         let v = self.to_vec();
//         v.hash(state);
//         self.to_vec().hash(state);
//     }
// }
