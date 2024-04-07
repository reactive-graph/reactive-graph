use crate::TypedReactivePropertyAccessor;
use crate::TypedReactivePropertyImpl;
use reactive_graph_reactive_model_api::ReactiveInstance;
use serde_json::json;

impl<IdType, ReactiveInstanceType> TypedReactivePropertyAccessor for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, f64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Target = f64;

    fn get(&self) -> Self::Target {
        self.reactive_instance.as_f64(&self.property_name).unwrap_or_default()
    }

    fn set(&self, value: Self::Target) {
        self.reactive_instance.set(&self.property_name, json!(value));
    }

    fn set_from<T: Into<Self::Target>>(&self, value: T) {
        self.reactive_instance.set(&self.property_name, json!(value.into()));
    }
}
