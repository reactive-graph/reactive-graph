use crate::ReactiveInstance;
use crate::TypedReactivePropertyImpl;
use serde_json::json;
use std::ops::Add;
use std::ops::AddAssign;

impl<IdType, ReactiveInstanceType, S> AddAssign<S> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
    S: Into<String>,
{
    fn add_assign(&mut self, rhs: S) {
        let rhs = rhs.into();
        if let Some(v) = self.reactive_instance.as_string(&self.property_name).map(|lhs| lhs.add(&rhs)) {
            self.reactive_instance.set(&self.property_name, json!(v));
        }
    }
}

impl<IdTypeSelf, ReactiveInstanceTypeSelf, IdTypeOther, ReactiveInstanceTypeOther>
    AddAssign<&TypedReactivePropertyImpl<IdTypeOther, ReactiveInstanceTypeOther, String>>
    for TypedReactivePropertyImpl<IdTypeSelf, ReactiveInstanceTypeSelf, String>
where
    IdTypeSelf: Clone,
    ReactiveInstanceTypeSelf: ReactiveInstance<IdTypeSelf>,
    IdTypeOther: Clone,
    ReactiveInstanceTypeOther: ReactiveInstance<IdTypeOther>,
{
    fn add_assign(&mut self, rhs: &TypedReactivePropertyImpl<IdTypeOther, ReactiveInstanceTypeOther, String>) {
        let Some(other) = rhs.reactive_instance.as_string(&rhs.property_name) else {
            return;
        };
        if let Some(v) = self.reactive_instance.as_string(&self.property_name).map(|lhs| lhs.add(&other)) {
            self.reactive_instance.set(&self.property_name, json!(v));
        }
    }
}
