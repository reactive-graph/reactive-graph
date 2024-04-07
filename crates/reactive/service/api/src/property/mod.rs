use std::marker::PhantomData;

pub use accessor::*;
pub use constructor::*;
pub use container::*;
pub use creator::*;
pub use eq::*;
pub use name::*;
pub use operator::*;
pub use property_string::accessor::*;
pub use property_string::callable::*;
pub use property_string::creator::*;
pub use property_string::display::*;
pub use property_string::eq::*;
pub use property_string::operator::*;
pub use property_u64::accessor::*;
pub use property_u64::callable::*;
pub use property_u64::creator::*;
pub use property_u64::display::*;
pub use property_u64::eq::*;
pub use property_u64::operator::*;

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
