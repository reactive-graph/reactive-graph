use inexor_rgf_reactive_model_api::ReactiveInstance;

pub trait TypedReactivePropertyCreator<IdType, ReactiveInstanceType>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Target;
    fn create(&self);
}
