use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_config_api::ConfigSystem;
use inexor_rgf_dynamic_graph_api::DynamicGraphSystem;
use inexor_rgf_graphql_api::GraphQLSystem;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_plugin_graphql_api::PluginGraphQLSystem;
use inexor_rgf_reactive_service_api::ReactiveSystem;
use inexor_rgf_runtime_graphql_api::RuntimeGraphQLSystem;
use inexor_rgf_type_system_api::TypeSystem;

use crate::GraphQLServer;
use crate::WebResourceManager;

#[injectable]
#[async_trait]
pub trait WebSystem: Lifecycle {
    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer + Send + Sync>;

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager + Send + Sync>;

    fn type_system(&self) -> Arc<dyn TypeSystem + Send + Sync>;

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync>;

    fn config_system(&self) -> Arc<dyn ConfigSystem + Send + Sync>;

    fn runtime_graphql_system(&self) -> Arc<dyn RuntimeGraphQLSystem + Send + Sync>;

    fn plugin_graphql_system(&self) -> Arc<dyn PluginGraphQLSystem + Send + Sync>;

    fn dynamic_graph_system(&self) -> Arc<dyn DynamicGraphSystem + Send + Sync>;

    fn graphql_system(&self) -> Arc<dyn GraphQLSystem + Send + Sync>;
}
