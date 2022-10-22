#![feature(register_tool)]
#![feature(test)]
#![register_tool(tarpaulin)]

pub use component::*;
pub use component_container::*;
pub use data_type::*;
pub use entity_instance::*;
pub use entity_type::*;
pub use extension::*;
pub use extension_container::*;
pub use flow_instance::*;
pub use flow_type::*;
pub use property_identifier::*;
pub use property_instance_accessor::*;
pub use property_type::*;
pub use reactive_behaviour_container::*;
pub use reactive_entity_instance::*;
pub use reactive_flow_instance::*;
pub use reactive_property_container::*;
pub use reactive_property_instance::*;
pub use reactive_relation_instance::*;
pub use relation_instance::*;
pub use relation_type::*;
pub use socket_type::*;
pub use type_container::*;
pub use types::*;

pub mod component;
pub mod component_container;
pub mod data_type;
pub mod entity_type;
pub mod extension;
pub mod extension_container;
pub mod flow_type;
pub mod property_identifier;
pub mod property_instance_accessor;
pub mod property_type;
pub mod reactive_behaviour_container;
pub mod reactive_property_container;
pub mod relation_type;
pub mod socket_type;
pub mod type_container;
pub mod types;

pub mod entity_instance;
pub mod flow_instance;
pub mod relation_instance;

pub mod reactive_entity_instance;
pub mod reactive_flow_instance;
pub mod reactive_property_instance;
pub mod reactive_relation_instance;

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
