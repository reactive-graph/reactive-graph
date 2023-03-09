#![feature(register_tool)]
#![register_tool(tarpaulin)]

pub use behaviour::*;
pub use entity::*;
pub use property::*;
pub use relation::*;

use inexor_rgf_core_model as model;

pub mod behaviour;
pub mod entity;
pub mod property;
pub mod relation;

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
