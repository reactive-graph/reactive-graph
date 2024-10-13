use crate::TypedReactivePropertyImpl;
use reactive_graph_reactive_model_api::ReactiveInstance;
use serde_json::json;

#[rustversion::nightly]
impl<IdType, ReactiveInstanceType> FnOnce<(u64,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Output = ();

    extern "rust-call" fn call_once(mut self, args: (u64,)) -> Self::Output {
        self.call_mut(args)
    }
}

#[rustversion::nightly]
impl<IdType, ReactiveInstanceType> FnMut<(u64,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    extern "rust-call" fn call_mut(&mut self, args: (u64,)) -> Self::Output {
        self.call(args)
    }
}

#[rustversion::nightly]
impl<IdType, ReactiveInstanceType> Fn<(u64,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    extern "rust-call" fn call(&self, args: (u64,)) -> Self::Output {
        self.reactive_instance.set(&self.property_name, json!(args.0));
    }
}

impl<IdType, ReactiveInstanceType> TypedReactivePropertyImpl<IdType, ReactiveInstanceType, u64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    pub fn op<F>(&self, f: F)
    where
        F: Fn(u64) -> u64,
    {
        if let Some(v) = self.reactive_instance.as_u64(&self.property_name) {
            self.reactive_instance.set(&self.property_name, json!(f(v)));
        }
    }
}
