use std::fmt::Display;

use serde_json::Value;

use reactive_graph_graph::ComponentContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstanceSetter;

use crate::ReactivePropertyContainer;

pub trait ReactiveInstanceUnidentifiable: ComponentContainer + PropertyInstanceSetter + NamespacedTypeGetter + Send + Sync {}

/// A reactive instance is a container for properties and components.
/// Furthermore, the reactive instance has a namespaced type.
pub trait ReactiveInstance<ID>: ReactiveInstanceUnidentifiable + ReactivePropertyContainer + Display + Clone {
    /// Returns the id of the reactive instance.
    fn id(&self) -> ID;
}

pub trait ReactiveInstanceGetter<T> {
    /// Returns the reactive instance.
    fn get_reactive_instance(&self) -> &T;
}

pub trait ReactiveInstanceContainer<ID: Clone, T: ReactiveInstance<ID>> {
    /// Returns the reactive instance of the behaviour.
    fn get_reactive_instance(&self) -> &T;

    fn get(&self, property_name: &str) -> Option<Value> {
        self.get_reactive_instance().get(property_name)
    }

    fn set(&self, property_name: &str, value: Value) {
        self.get_reactive_instance().set(property_name, value);
    }
}
