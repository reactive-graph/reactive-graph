use std::marker::PhantomData;

use uuid::Uuid;

use crate::TypedReactivePropertyCreator;
use crate::TypedReactivePropertyImpl;
use inexor_rgf_reactive_model_api::ReactiveInstance;

pub trait TypedReactivePropertyConstructor<IdType, ReactiveInstanceType>: TypedReactivePropertyCreator<IdType, ReactiveInstanceType>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type ID;

    type InstanceType: ReactiveInstance<Self::ID>;
    type Target;

    fn new(reactive_instance: Self::InstanceType, property_name: &str) -> Self;
}

impl<IdType, ReactiveInstanceType, Target> TypedReactivePropertyConstructor<IdType, ReactiveInstanceType>
    for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, Target>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType> + Clone,
    TypedReactivePropertyImpl<IdType, ReactiveInstanceType, Target>: TypedReactivePropertyCreator<IdType, ReactiveInstanceType>,
{
    type ID = IdType;
    type InstanceType = ReactiveInstanceType;
    type Target = Target;

    fn new(reactive_instance: Self::InstanceType, property_name: &str) -> Self {
        let property = Self {
            handle_id: Uuid::new_v4().as_u128(),
            reactive_instance: reactive_instance.clone(),
            property_name: property_name.to_string(),
            id_type: PhantomData,
            target: PhantomData,
        };
        if !reactive_instance.has_property(property_name) {
            property.create();
        }
        property
    }
}
