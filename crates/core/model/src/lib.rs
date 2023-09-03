#![feature(result_option_inspect)]
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
use inexor_rgf_test_utils as test_utils;

#[cfg(test)]
pub use instances::entities::entity_instance_tests;


pub mod types;
pub mod instances;
