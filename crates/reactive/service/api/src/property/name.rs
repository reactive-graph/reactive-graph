use crate::TypedReactivePropertyImpl;
use inexor_rgf_reactive_model_api::ReactiveInstance;

pub trait TypedReactivePropertyName {
    fn property_name(&self) -> String;
}

impl<IdType, ReactiveInstanceType, Target> TypedReactivePropertyName for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, Target>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn property_name(&self) -> String {
        self.property_name.clone()
    }
}
