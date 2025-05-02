use std::sync::Arc;

use async_graphql::EmptySubscription;
use async_graphql::Schema;
use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_graphql_api::PluginSchemaManager;
use reactive_graph_plugin_graphql_schema::PluginSchema;
use reactive_graph_plugin_graphql_schema::mutation::PluginMutation;
use reactive_graph_plugin_graphql_schema::query::PluginQuery;
use reactive_graph_plugin_service_api::PluginContainerManager;
use reactive_graph_plugin_service_api::PluginResolver;

#[derive(Component)]
pub struct PluginSchemaManagerImpl {
    plugin_container_manager: Arc<dyn PluginContainerManager + Send + Sync>,
    plugin_resolver: Arc<dyn PluginResolver + Send + Sync>, // Deferred<Arc<dyn PluginResolver + Send + Sync>>,
}

#[async_trait]
#[component_alias]
impl PluginSchemaManager for PluginSchemaManagerImpl {
    fn get_schema(&self) -> PluginSchema {
        Schema::build(PluginQuery, PluginMutation, EmptySubscription)
            .with_sorted_fields()
            .with_sorted_enums()
            .data(self.plugin_container_manager.clone())
            .data(self.plugin_resolver.clone())
            .finish()
    }
}

#[async_trait]
impl Lifecycle for PluginSchemaManagerImpl {
    async fn init(&self) {}

    async fn post_init(&self) {}

    async fn pre_shutdown(&self) {}

    async fn shutdown(&self) {}
}
