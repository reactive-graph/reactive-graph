use crate::PropertyType;
use serde_json::Value;

pub trait ReactivePropertyContainer {
    /// Sends the current value down the stream.
    fn tick(&self);

    /// Adds a reactive property with the given name and the given initial value.
    fn add_property<S: Into<String>>(&self, name: S, value: Value);

    /// Adds a reactive property with the given name and the given initial value.
    fn add_property_by_type(&self, property: &PropertyType);

    /// Removes the reactive property with the given name.
    fn remove_property<S: Into<String>>(&self, name: S);

    /// Observe the stream output flowing out of the stream of the property with the given
    /// name. The handle_id allows to remove the observer again.
    fn observe_with_handle<F>(&self, name: &str, subscriber: F, handle_id: u128)
    where
        F: FnMut(&Value) + 'static;

    /// Removes the subscriber with the given handle_id from the stream of the property with the
    /// given name.
    fn remove_observer(&self, name: &str, handle_id: u128);
}
