#![feature(lazy_cell)]

use inexor_rgf_config as config;

pub use component::*;
pub use entity::*;
pub use extension::*;
pub use instance_info::*;

pub mod component;
pub mod entity;
pub mod extension;
pub mod instance_info;

pub const NAMESPACE_CORE: &str = "core";
