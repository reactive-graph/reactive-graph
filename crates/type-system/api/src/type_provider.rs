pub trait TypeProvider<T: inexor_rgf_graph::NamespacedTypeContainer>: Send + Sync {
    /// Returns the id of the type provider.
    fn id<'a>(&self) -> &'a str;

    /// Returns a collection of types which should be registered.
    fn get_types(&self) -> T;

    /// Returns a collection of type ids which should be unregistered.
    fn get_type_ids(&self) -> T::TypeIds {
        self.get_types().type_ids()
    }
}
#[allow(unused_qualifications)]
impl<T: 'static> springtime_di::component::Injectable for dyn TypeProvider<T> + Sync + Send {}
