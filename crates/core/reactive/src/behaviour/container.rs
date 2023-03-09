use serde_json::Value;
use std::sync::Arc;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveInstance;

pub trait BehaviourTypeContainer {
    fn ty(&self) -> BehaviourTypeId;
}

pub trait BehaviourReactiveInstanceContainer<T: ReactiveInstance> {
    /// Returns the reactive instance of the behaviour.
    fn get_reactive_instance(&self) -> &Arc<T>;

    fn get(&self, property_name: &str) -> Option<Value> {
        self.get_reactive_instance().get(property_name)
    }

    fn set(&self, property_name: &str, value: Value) {
        self.get_reactive_instance().set(property_name, value);
    }
}
