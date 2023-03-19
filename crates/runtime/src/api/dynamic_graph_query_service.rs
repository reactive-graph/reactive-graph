use std::fmt;

use async_graphql::Response;
use async_trait::async_trait;
use serde_json::Error;

use crate::api::Lifecycle;

#[derive(Debug)]
pub enum DynamicQueryError {
    DynamicSchemaFailure,
    JsonError(Error),
}

impl fmt::Display for DynamicQueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DynamicQueryError::DynamicSchemaFailure => {
                write!(f, "Failed to generate the dynamic schema")
            }
            DynamicQueryError::JsonError(e) => {
                write!(f, "JsonError {e}")
            }
        }
    }
}

#[async_trait]
pub trait DynamicGraphQueryService: Send + Sync + Lifecycle {
    /// Runs the given GraphQL query.
    async fn query(&self, request: String) -> Result<String, DynamicQueryError>;

    /// Runs the given GraphQL query and returns the response.
    async fn query_response(&self, request: &str) -> Result<Response, DynamicQueryError>;
}
