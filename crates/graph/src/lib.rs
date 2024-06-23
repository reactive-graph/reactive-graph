#![feature(register_tool)]
#![register_tool(tarpaulin)]

pub use types::components::*;
pub use types::entities::*;
pub use types::extensions::*;
pub use types::flows::*;
pub use types::namespaces::*;
pub use types::properties::*;
pub use types::relations::*;
pub use types::type_id::*;
pub use types::variables::*;

pub use instances::components::*;
pub use instances::entities::*;
pub use instances::flows::*;
pub use instances::properties::*;
pub use instances::relations::*;

#[cfg(any(test, feature = "test"))]
use reactive_graph_test_utils as test_utils;

#[cfg(test)]
pub use instances::entities::entity_instance_tests;

pub mod instances;
pub mod types;

pub mod prelude {
    pub use crate::types::components::*;
    pub use crate::types::entities::*;
    pub use crate::types::extensions::*;
    pub use crate::types::flows::*;
    pub use crate::types::namespaces::*;
    pub use crate::types::properties::*;
    pub use crate::types::relations::*;
    pub use crate::types::type_id::*;
    pub use crate::types::variables::*;

    pub use crate::instances::components::*;
    pub use crate::instances::entities::*;
    pub use crate::instances::flows::*;
    pub use crate::instances::properties::*;
    pub use crate::instances::relations::*;
}
