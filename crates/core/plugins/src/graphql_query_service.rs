use async_trait::async_trait;
use serde_json::Error;

#[async_trait]
pub trait GraphQLQueryService: Send + Sync {
    /// Runs the given GraphQL query.
    async fn query(&self, request: String) -> Result<String, Error>;
}
