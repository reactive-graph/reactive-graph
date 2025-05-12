pub use instance::*;
pub use instance_address::*;

pub mod instance;
pub mod instance_address;
pub mod property_instance;
pub mod scalar;

#[allow(clippy::module_inception)]
pub mod schema {
    cynic::use_schema!("../../schema/graphql/reactive-graph-runtime-schema.graphql");
}
