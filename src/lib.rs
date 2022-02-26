#![feature(register_tool)]
#![feature(termination_trait_lib)]
#![feature(test)]
#![register_tool(tarpaulin)]

pub use component::*;
pub use data_type::*;
pub use entity_instance::*;
pub use entity_type::*;
pub use extension::*;
pub use flow::*;
pub use property_instance_accessor::*;
pub use property_type::*;
pub use reactive_entity_instance::*;
pub use reactive_flow::*;
pub use reactive_property_instance::*;
pub use reactive_relation_instance::*;
pub use relation_instance::*;
pub use relation_type::*;
pub use socket_type::*;

pub mod component;
pub mod data_type;
pub mod entity_type;
pub mod extension;
pub mod property_type;
pub mod relation_type;
pub mod socket_type;

pub mod entity_instance;
pub mod flow;
pub mod relation_instance;

pub mod reactive_entity_instance;
pub mod reactive_flow;
pub mod reactive_property_instance;
pub mod reactive_relation_instance;

pub mod property_instance_accessor;

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
