use crate::ReactiveInstance;
use crate::TypedReactivePropertyImpl;

impl<IdType, ReactiveInstanceType> PartialEq<u64> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &u64) -> bool {
        self.reactive_instance.as_u64(&self.property_name).map(|v| &v == other).unwrap_or_default()
    }
}

impl<IdType, ReactiveInstanceType> PartialEq<TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>> for u64
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>) -> bool {
        other.reactive_instance.as_u64(&other.property_name).map(|v| &v == self).unwrap_or_default()
    }
}
