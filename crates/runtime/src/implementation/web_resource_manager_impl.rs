use std::sync::Arc;
use std::sync::RwLock;

use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use async_trait::async_trait;
use dashmap::DashMap;
use log::debug;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::api::WebResourceManager;
use crate::plugins::WebResourceProvider;

#[wrapper]
pub struct WebResourceProviders(DashMap<String, Arc<dyn WebResourceProvider>>);

#[wrapper]
pub struct WebResourceProviderPaths(DashMap<Uuid, String>);

pub struct WebResourceManagerImpl {
    default_base_path: RwLock<Option<String>>,
    web_resource_providers: WebResourceProviders,
    web_resource_provider_paths: WebResourceProviderPaths,
}

#[component]
impl WebResourceManagerImpl {
    #[provides]
    fn new() -> Self {
        Self {
            default_base_path: RwLock::new(None),
            web_resource_providers: WebResourceProviders(DashMap::new()),
            web_resource_provider_paths: WebResourceProviderPaths(DashMap::new()),
        }
    }
}

#[async_trait]
#[provides]
impl WebResourceManager for WebResourceManagerImpl {
    fn has(&self, base_path: String) -> bool {
        self.get(base_path).is_some()
    }

    fn get(&self, base_path: String) -> Option<Arc<dyn WebResourceProvider>> {
        self.web_resource_providers.0.get(base_path.as_str()).map(|p| p.value().clone())
    }

    fn get_default(&self) -> Option<Arc<dyn WebResourceProvider>> {
        self.get_default_base_path()
            .and_then(|default_base_path| self.web_resource_providers.0.get(default_base_path.as_str()).map(|p| p.value().clone()))
    }

    fn get_default_base_path(&self) -> Option<String> {
        self.default_base_path.read().unwrap().clone()
    }

    fn add_provider(&self, id: Uuid, web_resource_provider: Arc<dyn WebResourceProvider>) {
        let base_path: String = web_resource_provider.get_base_path();
        debug!("Registering web resource provider with base path: {}", base_path);
        self.web_resource_providers.0.insert(base_path.clone(), web_resource_provider.clone());
        self.web_resource_provider_paths.0.insert(id, base_path);
    }

    fn remove_provider(&self, id: &Uuid) {
        if let Some(base_path) = self.web_resource_provider_paths.0.get(id) {
            self.web_resource_providers.0.remove(base_path.value());
        }
        self.web_resource_provider_paths.0.remove(id);
    }
}

impl Lifecycle for WebResourceManagerImpl {
    fn init(&self) {
        let graphql_server_config = crate::config::graphql::get_graphql_server_config();
        if let Some(default_base_path) = graphql_server_config.default_base_path {
            let mut writer = self.default_base_path.write().unwrap();
            writer.replace(default_base_path);
        }
    }

    fn shutdown(&self) {
        self.web_resource_provider_paths.0.clear();
        self.web_resource_providers.0.clear();
    }
}
