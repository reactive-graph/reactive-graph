use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::debug;
use uuid::Uuid;

use crate::api::ConfigManager;
use crate::api::Lifecycle;
use crate::api::WebResourceManager;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::plugins::WebResourceProvider;

#[wrapper]
pub struct WebResourceProviders(DashMap<String, Arc<dyn WebResourceProvider>>);

#[provides]
fn create_web_resource_providers() -> WebResourceProviders {
    WebResourceProviders(DashMap::new())
}

#[wrapper]
pub struct WebResourceProviderContextPaths(DashMap<Uuid, String>);

#[provides]
fn create_web_resource_provider_paths() -> WebResourceProviderContextPaths {
    WebResourceProviderContextPaths(DashMap::new())
}

#[component]
pub struct WebResourceManagerImpl {
    config_manager: Wrc<dyn ConfigManager>,

    web_resource_providers: WebResourceProviders,
    web_resource_provider_context_paths: WebResourceProviderContextPaths,
}

#[async_trait]
#[provides]
impl WebResourceManager for WebResourceManagerImpl {
    fn has(&self, context_path: String) -> bool {
        self.get(context_path).is_some()
    }

    fn get(&self, context_path: String) -> Option<Arc<dyn WebResourceProvider>> {
        self.web_resource_providers.0.get(context_path.as_str()).map(|p| p.value().clone())
    }

    fn get_default(&self) -> Option<Arc<dyn WebResourceProvider>> {
        self.config_manager
            .get_graphql_default_context_path()
            .and_then(|default_context_path| self.web_resource_providers.0.get(default_context_path.as_str()).map(|p| p.value().clone()))
    }

    async fn register_provider(&self, web_resource_provider: Arc<dyn WebResourceProvider>) {
        let id = web_resource_provider.id();
        let context_path: String = web_resource_provider.get_context_path();
        debug!("Registering web resource provider with context path: {}", context_path);
        self.web_resource_providers.0.insert(context_path.clone(), web_resource_provider.clone());
        self.web_resource_provider_context_paths.0.insert(id, context_path);
    }

    async fn unregister_provider(&self, id: Uuid) {
        if let Some(context_path) = self.web_resource_provider_context_paths.0.get(&id) {
            self.web_resource_providers.0.remove(context_path.value());
        }
        self.web_resource_provider_context_paths.0.remove(&id);
    }
}

#[async_trait]
impl Lifecycle for WebResourceManagerImpl {
    async fn shutdown(&self) {
        self.web_resource_provider_context_paths.0.clear();
        self.web_resource_providers.0.clear();
    }
}
