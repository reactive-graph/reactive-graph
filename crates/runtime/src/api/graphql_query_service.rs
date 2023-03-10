use async_trait::async_trait;
use serde_json::Error;

use crate::api::Lifecycle;

#[async_trait]
pub trait GraphQLQueryService: Send + Sync + Lifecycle {
    /// Runs the given GraphQL query.
    async fn query(&self, request: String) -> Result<String, Error>;
}
