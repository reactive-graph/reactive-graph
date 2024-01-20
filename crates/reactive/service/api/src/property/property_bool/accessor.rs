use crate::TypedReactivePropertyAccessor;
use crate::TypedReactivePropertyImpl;
use inexor_rgf_reactive_model_api::ReactiveInstance;
use serde_json::json;

impl<IdType, ReactiveInstanceType> TypedReactivePropertyAccessor for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, bool>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Target = bool;

    fn get(&self) -> Self::Target {
        self.reactive_instance.as_bool(&self.property_name).unwrap_or_default()
    }

    fn set(&self, value: Self::Target) {
        self.reactive_instance.set(&self.property_name, json!(value));
    }

    fn set_from<T: Into<Self::Target>>(&self, value: T) {
        self.reactive_instance.set(&self.property_name, json!(value.into()));
    }
}
