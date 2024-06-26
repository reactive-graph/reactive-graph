use crate::TypedReactivePropertyImpl;
use reactive_graph_reactive_model_api::ReactiveInstance;

impl<IdType, ReactiveInstanceType> PartialEq<bool> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, bool>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn eq(&self, other: &bool) -> bool {
        self.reactive_instance.as_bool(&self.property_name).map(|v| &v == other).unwrap_or_default()
    }
}
