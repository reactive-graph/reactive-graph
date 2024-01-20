use serde_json::Value;

use inexor_rgf_graph::Mutability;
use inexor_rgf_graph::PropertyType;

pub trait ReactivePropertyContainer {
    /// Sends the current value down the stream if mutable.
    fn tick_checked(&self);

    /// Sends the current value down the stream.
    fn tick(&self);

    /// Returns true, if a property with the given name exists.
    fn has_property(&self, name: &str) -> bool;

    /// Adds a reactive property with the given name and the given initial value.
    fn add_property<S: Into<String>>(&self, name: S, mutability: Mutability, value: Value);

    /// Adds a reactive property with the given name and the given initial value.
    fn add_property_by_type(&self, property: &PropertyType);

    /// Removes the reactive property with the given name.
    fn remove_property<S: Into<String>>(&self, name: S);

    /// Observe the stream output flowing out of the stream of the property with the given
    /// name. The handle_id allows to remove the observer again.
    fn observe_with_handle<F>(&self, name: &str, subscriber: F, handle_id: u128)
    where
        F: FnMut(&Value) + 'static + Send;

    /// Removes the subscriber with the given handle_id from the stream of the property with the
    /// given name.
    fn remove_observer(&self, name: &str, handle_id: u128);

    /// Removes the subscribers of the property with the given name.
    fn remove_observers(&self, name: &str);

    /// Removes all subscribers of all properties.
    fn remove_all_observers(&self);
}
