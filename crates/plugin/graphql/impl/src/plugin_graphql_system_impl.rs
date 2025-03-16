use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_graphql_api::PluginGraphQLSystem;
use reactive_graph_plugin_graphql_api::PluginQueryService;
use reactive_graph_plugin_graphql_api::PluginSchemaManager;

#[derive(Component)]
pub struct PluginGraphQLSystemImpl {
    plugin_query_service: Arc<dyn PluginQueryService + Send + Sync>,
    plugin_schema_manager: Arc<dyn PluginSchemaManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl PluginGraphQLSystem for PluginGraphQLSystemImpl {
    fn get_plugin_query_service(&self) -> Arc<dyn PluginQueryService + Send + Sync> {
        self.plugin_query_service.clone()
    }

    fn get_plugin_schema_manager(&self) -> Arc<dyn PluginSchemaManager + Send + Sync> {
        self.plugin_schema_manager.clone()
    }
}

#[async_trait]
impl Lifecycle for PluginGraphQLSystemImpl {
    async fn init(&self) {
        self.plugin_schema_manager.init().await;
        self.plugin_query_service.init().await;
    }

    async fn post_init(&self) {
        self.plugin_schema_manager.post_init().await;
        self.plugin_query_service.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.plugin_query_service.pre_shutdown().await;
        self.plugin_schema_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.plugin_query_service.shutdown().await;
        self.plugin_schema_manager.shutdown().await;
    }
}
