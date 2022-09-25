use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::graphql::InexorSchema;

#[async_trait]
pub trait GraphQLSchemaManager: Send + Sync + Lifecycle {
    /// Returns the GraphQL schema.
    fn get_schema(&self) -> InexorSchema;
}
