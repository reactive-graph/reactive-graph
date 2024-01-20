use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_plugin_graphql_schema::PluginSchema;

#[injectable]
#[async_trait]
pub trait PluginSchemaManager: Send + Sync + Lifecycle {
    /// Returns the GraphQL schema.
    fn get_schema(&self) -> PluginSchema;
}
