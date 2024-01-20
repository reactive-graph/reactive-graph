pub use plugin::*;

pub mod plugin;

#[allow(clippy::module_inception)]
pub mod schema {
    cynic::use_schema!("schema_plugin.graphql");
}
