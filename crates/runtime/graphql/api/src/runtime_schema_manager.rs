use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_runtime_graphql_schema::RuntimeSchema;

#[injectable]
#[async_trait]
pub trait RuntimeSchemaManager: Send + Sync + Lifecycle {
    /// Returns the GraphQL schema.
    fn get_schema(&self) -> RuntimeSchema;
}
