use crate::TypedReactivePropertyCreator;
use crate::TypedReactivePropertyImpl;
use inexor_rgf_graph::Mutability;
use inexor_rgf_reactive_model_api::ReactiveInstance;
use serde_json::json;

impl<IdType, ReactiveInstanceType> TypedReactivePropertyCreator<IdType, ReactiveInstanceType> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, f64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Target = String;

    fn create(&self) {
        if !self.reactive_instance.has_property(&self.property_name) {
            self.reactive_instance.add_property(&self.property_name, Mutability::Mutable, json!(0f64));
        }
    }
}
