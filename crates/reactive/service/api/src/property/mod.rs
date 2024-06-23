use std::marker::PhantomData;

pub use accessor::*;
pub use constructor::*;
pub use container::*;
pub use creator::*;
pub use name::*;

use reactive_graph_reactive_model_api::ReactiveInstance;

pub mod accessor;
pub mod constructor;
pub mod container;
pub mod creator;
pub mod eq;
pub mod name;
pub mod operator;
pub mod property_bool;
pub mod property_f64;
pub mod property_i64;
pub mod property_string;
pub mod property_u64;

pub struct TypedReactivePropertyImpl<IdType, ReactiveInstanceType, Target>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    handle_id: u128,
    reactive_instance: ReactiveInstanceType,
    property_name: String,
    id_type: PhantomData<IdType>,
    target: PhantomData<Target>,
}
