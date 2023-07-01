use inexor_rgf_core_model as model;

pub use component::*;
pub use entity::*;
pub use extension::*;
pub use instance_address::*;
pub use instance_info::*;

pub mod component;
pub mod entity;
pub mod extension;
pub mod instance_address;
pub mod instance_info;

pub const NAMESPACE_CORE: &str = "core";
