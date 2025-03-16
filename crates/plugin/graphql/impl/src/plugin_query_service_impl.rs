use std::sync::Arc;

use async_graphql::Request;
use async_graphql::Response;
use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_graphql_api::PluginQueryService;
use reactive_graph_plugin_graphql_api::PluginSchemaManager;

#[derive(Component)]
pub struct PluginQueryServiceImpl {
    plugin_schema_manager: Arc<dyn PluginSchemaManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl PluginQueryService for PluginQueryServiceImpl {
    async fn query(&self, request: &str) -> Result<String, serde_json::Error> {
        let schema = self.plugin_schema_manager.get_schema();
        let result = schema.execute(request).await;
        serde_json::to_string(&result)
    }

    async fn query_response(&self, request: &str) -> Response {
        self.plugin_schema_manager.get_schema().execute(request).await
    }

    async fn execute(&self, request: Request) -> Response {
        self.plugin_schema_manager.get_schema().execute(request).await
    }
}

#[async_trait]
impl Lifecycle for PluginQueryServiceImpl {}
