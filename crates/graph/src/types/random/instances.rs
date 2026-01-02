pub trait RandomInstance: Sized {
    type Error;
    type TypeId;
    fn random_instance() -> Result<Self, Self::Error>;
    fn random_instance_with_id(ty: &Self::TypeId) -> Result<Self, Self::Error>;
}

pub trait RandomInstances: Sized {
    type Error;
    fn random_instances() -> Result<Self, Self::Error>;
}
