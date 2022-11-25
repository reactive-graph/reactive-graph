use serde_json::Value;

/// A PropertyObserverContainer manages the observers of a PropertyContainer.
///
/// Internally it stores the handle ids of created observers. This makes it possible to remove the
/// observers for a single property by name or for all properties.
pub trait PropertyObserverContainer {
    /// Observes the property with the given name.
    /// A handle will be automatically created and stored
    fn observe_with_handle<F>(&self, name: &str, subscriber: F) -> u128
    where
        F: FnMut(&Value) + 'static;

    /// Propagates the value from the property with the given name to the target property with the given name.
    fn propagate(&self, name: &str, target_property_name: &str);

    /// Removes the observers of the property with the given name and the given observer handle.
    fn remove_observer(&self, name: &str, handle_id: u128);

    /// Removes all observers of the property with the given name that are managed by this PropertyObserverContainer.
    fn remove_observers(&self, name: &str);

    /// Removes all observers that are managed by this PropertyObserverContainer.
    fn remove_all_observers(&self);
}
