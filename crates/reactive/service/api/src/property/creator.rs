use reactive_graph_reactive_model_api::ReactiveInstance;

pub trait TypedReactivePropertyCreator<IdType, ReactiveInstanceType>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Target;
    fn create(&self);
}
