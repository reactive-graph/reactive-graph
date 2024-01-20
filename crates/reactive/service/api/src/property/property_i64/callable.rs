use crate::TypedReactivePropertyImpl;
use inexor_rgf_reactive_model_api::ReactiveInstance;
use serde_json::json;

impl<IdType, ReactiveInstanceType> FnOnce<(i64,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, i64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Output = ();

    extern "rust-call" fn call_once(mut self, args: (i64,)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<IdType, ReactiveInstanceType> FnMut<(i64,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, i64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    extern "rust-call" fn call_mut(&mut self, args: (i64,)) -> Self::Output {
        self.call(args)
    }
}

impl<IdType, ReactiveInstanceType> Fn<(i64,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, i64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    extern "rust-call" fn call(&self, args: (i64,)) -> Self::Output {
        self.reactive_instance.set(&self.property_name, json!(args.0));
    }
}

impl<IdType, ReactiveInstanceType> TypedReactivePropertyImpl<IdType, ReactiveInstanceType, i64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    pub fn op<F>(&self, f: F)
    where
        F: Fn(i64) -> i64,
    {
        if let Some(v) = self.reactive_instance.as_i64(&self.property_name) {
            self.reactive_instance.set(&self.property_name, json!(f(v)));
        }
    }
}
