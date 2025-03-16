use std::sync::Arc;

use async_graphql::Request;
use async_graphql::Response;
use async_trait::async_trait;
use log::info;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graphql_api::GraphQLQueryService;
use reactive_graph_graphql_api::GraphQLSchemaManager;
use reactive_graph_lifecycle::Lifecycle;

#[derive(Component)]
pub struct GraphQLQueryServiceImpl {
    graphql_schema_manager: Arc<dyn GraphQLSchemaManager + Send + Sync>,
}

impl GraphQLQueryServiceImpl {}

#[async_trait]
#[component_alias]
impl GraphQLQueryService for GraphQLQueryServiceImpl {
    async fn query(&self, request: &str) -> Result<String, serde_json::Error> {
        info!("Run query: {request}");
        let schema = self.graphql_schema_manager.get_schema();
        let result = schema.execute(request).await;
        serde_json::to_string(&result)
    }

    async fn query_response(&self, request: &str) -> Response {
        self.graphql_schema_manager.get_schema().execute(request).await
    }

    async fn execute(&self, request: Request) -> Response {
        self.graphql_schema_manager.get_schema().execute(request).await
    }
}

#[async_trait]
impl Lifecycle for GraphQLQueryServiceImpl {
    async fn post_init(&self) {}
}
