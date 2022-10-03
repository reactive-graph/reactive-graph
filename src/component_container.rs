use crate::Component;

/// Reactive instance container for components.
pub trait ComponentContainer {
    /// Adds a component to the container.
    fn add_component<S: Into<String>>(&self, component: S);

    /// Adds a component to the container and initializes the reactive property instances.
    fn add_component_with_properties(&self, component: &Component);

    /// Removes a component from the container.
    fn remove_component<S: Into<String>>(&self, component: S);

    /// Returns true, if the reactive instance is composed with the given component.
    fn is_a<S: Into<String>>(&self, component: S) -> bool;
}
