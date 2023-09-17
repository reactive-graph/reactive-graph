use crate::ReactiveInstance;
use crate::TypedReactivePropertyImpl;

impl<IdType, ReactiveInstanceType> PartialEq<f64> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, f64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &f64) -> bool {
        self.reactive_instance.as_f64(&self.property_name).map(|v| &v == other).unwrap_or_default()
    }
}

impl<IdType, ReactiveInstanceType> PartialEq<TypedReactivePropertyImpl<IdType, ReactiveInstanceType, f64>> for f64
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &TypedReactivePropertyImpl<IdType, ReactiveInstanceType, f64>) -> bool {
        other.reactive_instance.as_f64(&other.property_name).map(|v| &v == self).unwrap_or_default()
    }
}
