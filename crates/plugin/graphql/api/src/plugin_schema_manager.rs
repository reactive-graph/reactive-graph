use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_graphql_schema::PluginSchema;

#[injectable]
#[async_trait]
pub trait PluginSchemaManager: Send + Sync + Lifecycle {
    /// Returns the GraphQL schema.
    fn get_schema(&self) -> PluginSchema;
}
