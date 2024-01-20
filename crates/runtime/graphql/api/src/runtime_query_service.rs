use async_graphql::Request;
use async_graphql::Response;
use async_trait::async_trait;
use serde_json::Error;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait RuntimeQueryService: Send + Sync + Lifecycle {
    /// Runs the given GraphQL query.
    async fn query(&self, request: &str) -> Result<String, Error>;

    /// Runs the given GraphQL query and returns the response.
    async fn query_response(&self, request: &str) -> Response;

    /// Executes the given GraphQL request and returns the GraphQL response.
    async fn execute(&self, request: Request) -> Response;
}
