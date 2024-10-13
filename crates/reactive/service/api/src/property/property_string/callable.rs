use crate::TypedReactivePropertyImpl;
use reactive_graph_reactive_model_api::ReactiveInstance;
use serde_json::json;

#[rustversion::nightly]
impl<IdType, ReactiveInstanceType> FnOnce<(String,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Output = ();

    extern "rust-call" fn call_once(mut self, args: (String,)) -> Self::Output {
        self.call_mut(args)
    }
}

#[rustversion::nightly]
impl<IdType, ReactiveInstanceType> FnMut<(String,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    extern "rust-call" fn call_mut(&mut self, args: (String,)) -> Self::Output {
        self.call(args)
    }
}

#[rustversion::nightly]
impl<IdType, ReactiveInstanceType> Fn<(String,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    extern "rust-call" fn call(&self, args: (String,)) -> Self::Output {
        self.reactive_instance.set(&self.property_name, json!(args.0));
    }
}

impl<IdType, ReactiveInstanceType> TypedReactivePropertyImpl<IdType, ReactiveInstanceType, String>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    pub fn op<F>(&self, f: F)
    where
        F: Fn(String) -> String,
    {
        if let Some(v) = self.reactive_instance.as_string(&self.property_name).map(f).map(|v| json!(v)) {
            self.reactive_instance.set(&self.property_name, v);
        }
    }
}
