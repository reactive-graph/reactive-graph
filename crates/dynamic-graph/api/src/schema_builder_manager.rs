use async_graphql::dynamic::SchemaBuilder;
use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait SchemaBuilderManager: Send + Sync + Lifecycle {
    /// Returns a new schema builder.
    fn get_schema_builder(&self) -> SchemaBuilder;
}
