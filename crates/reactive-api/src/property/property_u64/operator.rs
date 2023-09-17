use crate::ReactiveInstance;
use crate::TypedReactivePropertyImpl;
use serde_json::json;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Sub;
use std::ops::SubAssign;

impl<IdType, ReactiveInstanceType> AddAssign<u64> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn add_assign(&mut self, rhs: u64) {
        if let Some(v) = self.reactive_instance.as_u64(&self.property_name).map(|lhs| lhs.add(rhs)) {
            self.reactive_instance.set(&self.property_name, json!(v));
        }
    }
}

impl<IdType, ReactiveInstanceType> SubAssign<u64> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn sub_assign(&mut self, rhs: u64) {
        if let Some(v) = self.reactive_instance.as_u64(&self.property_name).map(|lhs| lhs.sub(rhs)) {
            self.reactive_instance.set(&self.property_name, json!(v));
        }
    }
}

impl<IdType, ReactiveInstanceType> MulAssign<u64> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn mul_assign(&mut self, rhs: u64) {
        if let Some(v) = self.reactive_instance.as_u64(&self.property_name).map(|lhs| lhs.mul(rhs)) {
            self.reactive_instance.set(&self.property_name, json!(v));
        }
    }
}

impl<IdType, ReactiveInstanceType> DivAssign<u64> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn div_assign(&mut self, rhs: u64) {
        if rhs == 0 {
            return;
        }
        if let Some(v) = self.reactive_instance.as_u64(&self.property_name).map(|lhs| lhs.div(rhs)) {
            self.reactive_instance.set(&self.property_name, json!(v));
        }
    }
}

impl<IdType, ReactiveInstanceType> RemAssign<u64> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn rem_assign(&mut self, rhs: u64) {
        if rhs == 0 {
            return;
        }
        if let Some(v) = self.reactive_instance.as_u64(&self.property_name).map(|lhs| lhs.rem(rhs)) {
            self.reactive_instance.set(&self.property_name, json!(v));
        }
    }
}
