#![feature(register_tool)]
#![feature(test)]
#![register_tool(tarpaulin)]

pub use behaviour_type_id::*;
pub use components::*;
pub use entities::*;
pub use flows::*;
pub use properties::*;
pub use reactive_behaviour_container::*;
pub use reactive_instance::*;
pub use relations::*;

use inexor_rgf_core_model as model;

pub mod behaviour_type_id;
pub mod components;
pub mod entities;
pub mod flows;
pub mod properties;
pub mod reactive_behaviour_container;
pub mod reactive_instance;
pub mod relations;

#[cfg(test)]
mod test_utils;
