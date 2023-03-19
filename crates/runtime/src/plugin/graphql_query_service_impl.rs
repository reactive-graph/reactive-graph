use async_trait::async_trait;

use serde_json::Error;
use std::sync::Arc;

use crate::plugins::GraphQLQueryService;

pub struct GraphQLQueryServiceImpl {
    graphql_query_service: Arc<dyn crate::api::GraphQLQueryService>,
}

impl GraphQLQueryServiceImpl {
    pub fn new(graphql_query_service: Arc<dyn crate::api::GraphQLQueryService>) -> Self {
        Self { graphql_query_service }
    }
}

#[async_trait]
impl GraphQLQueryService for GraphQLQueryServiceImpl {
    async fn query(&self, request: &str) -> Result<String, Error> {
        self.graphql_query_service.query(request).await
    }
}
