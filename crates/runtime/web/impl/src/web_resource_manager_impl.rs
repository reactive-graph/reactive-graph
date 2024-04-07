use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::debug;
use uuid::Uuid;

use reactive_graph_config_api::ConfigManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_api::WebResourceProvider;
use reactive_graph_runtime_web_api::WebResourceManager;
use springtime_di::component_alias;
use springtime_di::Component;

#[derive(Component)]
pub struct WebResourceManagerImpl {
    config_manager: Arc<dyn ConfigManager + Send + Sync>,

    #[component(default = "DashMap::new")]
    web_resource_providers: DashMap<String, Arc<dyn WebResourceProvider>>, // WebResourceProviders,
    #[component(default = "DashMap::new")]
    web_resource_provider_context_paths: DashMap<Uuid, String>, // WebResourceProviderContextPaths,
}

#[async_trait]
#[component_alias]
impl WebResourceManager for WebResourceManagerImpl {
    fn has(&self, context_path: String) -> bool {
        self.get(context_path).is_some()
    }

    fn get(&self, context_path: String) -> Option<Arc<dyn WebResourceProvider>> {
        self.web_resource_providers.get(context_path.as_str()).map(|p| p.value().clone())
    }

    fn get_default(&self) -> Option<Arc<dyn WebResourceProvider>> {
        self.config_manager
            .get_graphql_default_context_path()
            .and_then(|default_context_path| self.web_resource_providers.get(default_context_path.as_str()).map(|p| p.value().clone()))
    }

    async fn register_provider(&self, web_resource_provider: Arc<dyn WebResourceProvider>) {
        let id = web_resource_provider.id();
        let context_path: String = web_resource_provider.get_context_path();
        debug!("Registering web resource provider with context path: {}", context_path);
        self.web_resource_providers.insert(context_path.clone(), web_resource_provider.clone());
        self.web_resource_provider_context_paths.insert(id, context_path);
    }

    async fn unregister_provider(&self, id: Uuid) {
        if let Some(context_path) = self.web_resource_provider_context_paths.get(&id) {
            self.web_resource_providers.remove(context_path.value());
        }
        self.web_resource_provider_context_paths.remove(&id);
    }
}

#[async_trait]
impl Lifecycle for WebResourceManagerImpl {
    async fn shutdown(&self) {
        self.web_resource_provider_context_paths.clear();
        self.web_resource_providers.clear();
    }
}
