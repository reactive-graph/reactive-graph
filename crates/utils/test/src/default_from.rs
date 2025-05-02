pub trait DefaultFrom<T> {
    fn default_from(ty: &T) -> Self;
}
