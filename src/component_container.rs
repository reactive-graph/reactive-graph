pub trait ComponentContainer {
    fn add_component<S: Into<String>>(&self, component: S);

    fn remove_component<S: Into<String>>(&self, component: S);

    /// Returns true, if the reactive instance is composed with the given component.
    fn is_a<S: Into<String>>(&self, component: S) -> bool;
}
