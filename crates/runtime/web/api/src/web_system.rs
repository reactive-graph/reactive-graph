use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_config_api::ConfigSystem;
use reactive_graph_dynamic_graph_api::DynamicGraphSystem;
use reactive_graph_graphql_api::GraphQLSystem;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_graphql_api::PluginGraphQLSystem;
use reactive_graph_reactive_service_api::ReactiveSystem;
use reactive_graph_runtime_graphql_api::RuntimeGraphQLSystem;
use reactive_graph_type_system_api::TypeSystemSystem;

use crate::GraphQLServer;
use crate::WebResourceManager;

#[injectable]
#[async_trait]
pub trait WebSystem: Lifecycle {
    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer + Send + Sync>;

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager + Send + Sync>;

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync>;

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync>;

    fn config_system(&self) -> Arc<dyn ConfigSystem + Send + Sync>;

    fn runtime_graphql_system(&self) -> Arc<dyn RuntimeGraphQLSystem + Send + Sync>;

    fn plugin_graphql_system(&self) -> Arc<dyn PluginGraphQLSystem + Send + Sync>;

    fn dynamic_graph_system(&self) -> Arc<dyn DynamicGraphSystem + Send + Sync>;

    fn graphql_system(&self) -> Arc<dyn GraphQLSystem + Send + Sync>;
}
