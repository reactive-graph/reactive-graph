use async_trait::async_trait;

use reactive_graph_graphql_api::GraphQLQueryService;
use serde_json::Error;
use std::sync::Arc;

pub struct GraphQLQueryServiceDelegate {
    graphql_query_service: Arc<dyn GraphQLQueryService + Send + Sync>,
}

impl GraphQLQueryServiceDelegate {
    pub fn new(graphql_query_service: Arc<dyn GraphQLQueryService + Send + Sync>) -> Self {
        Self { graphql_query_service }
    }
}

#[async_trait]
impl reactive_graph_plugin_api::GraphQLQueryService for GraphQLQueryServiceDelegate {
    async fn query(&self, request: &str) -> Result<String, Error> {
        self.graphql_query_service.query(request).await
    }
}
