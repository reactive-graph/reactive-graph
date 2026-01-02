pub trait DefaultFrom<T> {
    fn default_from(ty: T) -> Self;
}

pub trait DefaultTryFrom<T>: Sized {
    type Error;
    fn default_try_from(ty: T) -> Result<Self, Self::Error>;
}
