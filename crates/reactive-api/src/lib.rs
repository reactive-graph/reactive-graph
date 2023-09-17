#![feature(unboxed_closures)]
#![feature(fn_traits)]

pub use component_container::*;
pub use entity::*;
pub use error::entities::*;
pub use error::flows::*;
pub use error::relations::*;
#[cfg(feature = "derive")]
pub use inexor_rgf_reactive_derive::reactive_entity;
#[cfg(feature = "derive")]
pub use inexor_rgf_reactive_derive::ReactiveEntity;
pub use instance::*;
pub use property::*;
pub use reactive_property_container::*;
pub use relation::*;

use inexor_rgf_graph as model;

pub mod component_container;
pub mod entity;
pub mod error;
pub mod instance;
pub mod property;
pub mod reactive_property_container;
pub mod relation;

pub mod prelude {
    pub use crate::component_container::*;
    pub use crate::entity::*;
    pub use crate::error::*;
    pub use crate::instance::*;
    pub use crate::property::*;
    pub use crate::reactive_property_container::*;
    pub use crate::relation::*;
}
