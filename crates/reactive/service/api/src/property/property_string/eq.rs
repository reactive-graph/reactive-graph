use crate::TypedReactivePropertyImpl;
use inexor_rgf_reactive_model_api::ReactiveInstance;

impl<IdType, ReactiveInstanceType> PartialEq<&str> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &&str) -> bool {
        self.reactive_instance.as_string(&self.property_name).map(|v| &v == other).unwrap_or_default()
    }
}

impl<IdType, ReactiveInstanceType> PartialEq<TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>> for &str
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>) -> bool {
        other.reactive_instance.as_string(&other.property_name).map(|v| &v == self).unwrap_or_default()
    }
}

impl<IdType, ReactiveInstanceType> PartialEq<String> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &String) -> bool {
        self.reactive_instance.as_string(&self.property_name).map(|v| &v == other).unwrap_or_default()
    }
}

impl<IdType, ReactiveInstanceType> PartialEq<TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>> for String
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>) -> bool {
        other.reactive_instance.as_string(&other.property_name).map(|v| &v == self).unwrap_or_default()
    }
}
