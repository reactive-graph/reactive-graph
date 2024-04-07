use crate::TypedReactivePropertyCreator;
use crate::TypedReactivePropertyImpl;
use reactive_graph_graph::Mutability;
use reactive_graph_reactive_model_api::ReactiveInstance;
use serde_json::Value::Bool;

impl<IdType, ReactiveInstanceType> TypedReactivePropertyCreator<IdType, ReactiveInstanceType> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, bool>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Target = String;

    fn create(&self) {
        if !self.reactive_instance.has_property(&self.property_name) {
            self.reactive_instance.add_property(&self.property_name, Mutability::Mutable, Bool(false));
        }
    }
}
