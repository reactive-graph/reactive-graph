use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_graphql_schema::InexorSchema;
use inexor_rgf_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait GraphQLSchemaManager: Send + Sync + Lifecycle {
    /// Returns the GraphQL schema.
    fn get_schema(&self) -> InexorSchema;
}
