use crate::Namespace;
use std::ops::Range;

pub trait RandomNamespacedType: Sized {
    type Error;
    type TypeId;

    fn random_type() -> Result<Self, Self::Error>;

    fn random_type_with_id(ty: &Self::TypeId) -> Result<Self, Self::Error>;
}

pub trait RandomChildType: Sized {
    type Error;
    fn random_child_type(namespace: &Namespace) -> Result<Self, Self::Error>;
}

pub trait RandomNamespacedTypes: Sized {
    type Error;

    fn random_types(range: Range<usize>) -> Result<Self, Self::Error>;
}

pub trait RandomNamespacedTypesWithId: RandomNamespacedTypes + Sized {
    type TypeIds;

    fn random_types_with_ids(ty: &Self::TypeIds) -> Result<Self, <Self as RandomNamespacedTypes>::Error>;
}

pub trait RandomChildTypes: Sized {
    type Error;
    fn random_child_types(namespace: &Namespace, range: Range<usize>) -> Result<Self, Self::Error>;
}
