use async_trait::async_trait;
use serde_json::Error;

use crate::api::Lifecycle;

pub enum DynamicQueryError {
    DynamicSchemaFailure,
    JsonError(Error),
}

#[async_trait]
pub trait DynamicGraphQueryService: Send + Sync + Lifecycle {
    /// Runs the given GraphQL query.
    async fn query(&self, request: String) -> Result<String, DynamicQueryError>;
}
