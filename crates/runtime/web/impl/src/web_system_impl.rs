use std::sync::Arc;

use async_trait::async_trait;
use reactive_graph_config_api::ConfigSystem;
use reactive_graph_dynamic_graph_api::DynamicGraphSystem;
use reactive_graph_graphql_api::GraphQLSystem;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_graphql_api::PluginGraphQLSystem;
use reactive_graph_reactive_service_api::ReactiveSystem;
use reactive_graph_runtime_graphql_api::RuntimeGraphQLSystem;
use reactive_graph_runtime_web_api::GraphQLServer;
use reactive_graph_runtime_web_api::WebResourceManager;
use reactive_graph_runtime_web_api::WebSystem;
use reactive_graph_type_system_api::TypeSystem;

#[derive(Component)]
pub struct WebSystemImpl {
    graphql_server: Arc<dyn GraphQLServer + Send + Sync>,
    web_resource_manager: Arc<dyn WebResourceManager + Send + Sync>,
    type_system: Arc<dyn TypeSystem + Send + Sync>,
    reactive_system: Arc<dyn ReactiveSystem + Send + Sync>,
    config_system: Arc<dyn ConfigSystem + Send + Sync>,
    runtime_graphql_system: Arc<dyn RuntimeGraphQLSystem + Send + Sync>,
    plugin_graphql_system: Arc<dyn PluginGraphQLSystem + Send + Sync>,
    graphql_system: Arc<dyn GraphQLSystem + Send + Sync>,
    dynamic_graph_system: Arc<dyn DynamicGraphSystem + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl WebSystem for WebSystemImpl {
    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer + Send + Sync> {
        self.graphql_server.clone()
    }

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager + Send + Sync> {
        self.web_resource_manager.clone()
    }

    fn type_system(&self) -> Arc<dyn TypeSystem + Send + Sync> {
        self.type_system.clone()
    }

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync> {
        self.reactive_system.clone()
    }

    fn config_system(&self) -> Arc<dyn ConfigSystem + Send + Sync> {
        self.config_system.clone()
    }

    fn runtime_graphql_system(&self) -> Arc<dyn RuntimeGraphQLSystem + Send + Sync> {
        self.runtime_graphql_system.clone()
    }

    fn plugin_graphql_system(&self) -> Arc<dyn PluginGraphQLSystem + Send + Sync> {
        self.plugin_graphql_system.clone()
    }

    fn dynamic_graph_system(&self) -> Arc<dyn DynamicGraphSystem + Send + Sync> {
        self.dynamic_graph_system.clone()
    }

    fn graphql_system(&self) -> Arc<dyn GraphQLSystem + Send + Sync> {
        self.graphql_system.clone()
    }
}

#[async_trait]
impl Lifecycle for WebSystemImpl {
    async fn init(&self) {
        self.web_resource_manager.init().await;
        self.graphql_server.init().await;
    }

    async fn post_init(&self) {
        self.web_resource_manager.post_init().await;
        self.graphql_server.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.graphql_server.pre_shutdown().await;
        self.web_resource_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.graphql_server.shutdown().await;
        self.web_resource_manager.shutdown().await;
    }
}
