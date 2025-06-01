use crate::TypedReactivePropertyImpl;
use reactive_graph_reactive_model_api::ReactiveInstance;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

impl<IdType, ReactiveInstanceType> Debug for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, i64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.reactive_instance.as_i64(&self.property_name) {
            None => Err(Error),
            Some(v) => {
                write!(f, "{v:?}")
            }
        }
    }
}

impl<IdType, ReactiveInstanceType> Display for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, i64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.reactive_instance.as_i64(&self.property_name) {
            None => Err(Error),
            Some(v) => {
                write!(f, "{v}")
            }
        }
    }
}
