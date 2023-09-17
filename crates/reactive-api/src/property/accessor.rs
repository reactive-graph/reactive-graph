pub trait TypedReactivePropertyAccessor {
    type Target;
    fn get(&self) -> Self::Target;

    fn set(&self, value: Self::Target);

    fn set_from<T: Into<Self::Target>>(&self, value: T);
}
