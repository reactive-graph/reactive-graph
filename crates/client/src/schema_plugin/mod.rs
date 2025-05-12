pub use plugin::*;

pub mod plugin;

#[allow(clippy::module_inception)]
pub mod schema {
    cynic::use_schema!("../../schema/graphql/reactive-graph-plugin-schema.graphql");
}
