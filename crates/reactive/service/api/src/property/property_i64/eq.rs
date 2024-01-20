use crate::TypedReactivePropertyImpl;
use inexor_rgf_reactive_model_api::ReactiveInstance;

impl<IdType, ReactiveInstanceType> PartialEq<i64> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, i64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &i64) -> bool {
        self.reactive_instance.as_i64(&self.property_name).map(|v| &v == other).unwrap_or_default()
    }
}

impl<IdType, ReactiveInstanceType> PartialEq<TypedReactivePropertyImpl<IdType, ReactiveInstanceType, i64>> for i64
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &TypedReactivePropertyImpl<IdType, ReactiveInstanceType, i64>) -> bool {
        other.reactive_instance.as_i64(&other.property_name).map(|v| &v == self).unwrap_or_default()
    }
}
