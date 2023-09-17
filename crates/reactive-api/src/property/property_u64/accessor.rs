use crate::ReactiveInstance;
use crate::TypedReactivePropertyAccessor;
use crate::TypedReactivePropertyImpl;
use serde_json::json;

impl<IdType, ReactiveInstanceType> TypedReactivePropertyAccessor for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Target = u64;

    fn get(&self) -> Self::Target {
        self.reactive_instance.as_u64(&self.property_name).unwrap_or_default()
    }

    fn set(&self, value: Self::Target) {
        self.reactive_instance.set(&self.property_name, json!(value));
    }

    fn set_from<T: Into<Self::Target>>(&self, value: T) {
        self.reactive_instance.set(&self.property_name, json!(value.into()));
    }
}
