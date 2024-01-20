use async_graphql::Response;
use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;

use crate::DynamicQueryError;

#[injectable]
#[async_trait]
pub trait DynamicGraphQueryService: Send + Sync + Lifecycle {
    /// Runs the given GraphQL query.
    async fn query(&self, request: String) -> Result<String, DynamicQueryError>;

    /// Runs the given GraphQL query and returns the response.
    async fn query_response(&self, request: &str) -> Result<Response, DynamicQueryError>;
}
