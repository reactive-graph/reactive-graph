#![feature(register_tool)]
#![register_tool(tarpaulin)]

use inexor_rgf_core_model as model;

pub mod entity;
pub use entity::*;

pub mod relation;
pub use relation::*;

pub mod property;
pub use property::*;

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
