use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_graphql_schema::ReactiveGraphSchema;
use reactive_graph_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait GraphQLSchemaManager: Send + Sync + Lifecycle {
    /// Returns the GraphQL schema.
    fn get_schema(&self) -> ReactiveGraphSchema;
}
