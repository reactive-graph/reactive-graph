#![feature(register_tool)]
#![register_tool(tarpaulin)]

pub mod entity;
pub use entity::*;

pub mod relation;
pub use relation::*;

pub mod property;
pub use property::*;

#[derive(Debug)]
pub struct BehaviourCreationError;

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
