use crate::Namespace;

pub trait RandomNamespacedTypeId: Sized {
    type Error;
    fn random_type_id() -> Result<Self, Self::Error>;
}

pub trait RandomChildTypeId: Sized {
    type Error;
    fn random_child_type_id(namespace: &Namespace) -> Result<Self, Self::Error>;
}

pub trait RandomNamespacedTypeIds: Sized {
    type Error;
    fn random_type_ids() -> Result<Self, Self::Error>;
}

pub trait RandomChildTypeIds: Sized {
    type Error;
    fn random_child_type_ids(namespace: &Namespace) -> Result<Self, Self::Error>;
}
