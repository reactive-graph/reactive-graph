use std::sync::Arc;

use async_graphql::Request;
use async_graphql::Response;
use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_runtime_graphql_api::RuntimeQueryService;
use reactive_graph_runtime_graphql_api::RuntimeSchemaManager;

#[derive(Component)]
pub struct RuntimeQueryServiceImpl {
    runtime_schema_manager: Arc<dyn RuntimeSchemaManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RuntimeQueryService for RuntimeQueryServiceImpl {
    async fn query(&self, request: &str) -> Result<String, serde_json::Error> {
        let schema = self.runtime_schema_manager.get_schema();
        let result = schema.execute(request).await;
        serde_json::to_string(&result)
    }

    async fn query_response(&self, request: &str) -> Response {
        self.runtime_schema_manager.get_schema().execute(request).await
    }

    async fn execute(&self, request: Request) -> Response {
        self.runtime_schema_manager.get_schema().execute(request).await
    }
}

#[async_trait]
impl Lifecycle for RuntimeQueryServiceImpl {}
