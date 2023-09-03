use serde_json::Value;

use crate::reactive::BehaviourTypeId;
use crate::reactive::ReactiveInstance;

pub trait BehaviourTypeContainer {
    fn ty(&self) -> BehaviourTypeId;
}

pub trait BehaviourReactiveInstanceContainer<ID: Clone, T: ReactiveInstance<ID>> {
    /// Returns the reactive instance of the behaviour.
    fn get_reactive_instance(&self) -> &T;

    fn get(&self, property_name: &str) -> Option<Value> {
        self.get_reactive_instance().get(property_name)
    }

    fn set(&self, property_name: &str, value: Value) {
        self.get_reactive_instance().set(property_name, value);
    }
}
