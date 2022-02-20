#![feature(register_tool)]
#![register_tool(tarpaulin)]

pub use component_builder::*;
pub use entity_instance_builder::*;
pub use entity_type_builder::*;
pub use flow_builder::*;
pub use reactive_entity_instance_builder::*;
pub use reactive_relation_instance_builder::*;
pub use relation_instance_builder::*;
pub use relation_type_builder::*;

pub mod component_builder;
pub mod entity_instance_builder;
pub mod entity_type_builder;
pub mod flow_builder;
pub mod reactive_entity_instance_builder;
pub mod reactive_relation_instance_builder;
pub mod relation_instance_builder;
pub mod relation_type_builder;

use inexor_rgf_core_model as model;

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
