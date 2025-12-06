use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaError;
use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait DynamicGraphSchemaBuilder: Send + Sync + Lifecycle {
    /// Builds the schema of the DynamicGraph.
    fn build_dynamic_schema(&self) -> Result<Schema, SchemaError>;
}
