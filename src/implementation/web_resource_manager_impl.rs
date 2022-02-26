use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::di::{component, provides, wrapper};
use async_trait::async_trait;
use log::debug;

use crate::api::Lifecycle;
use crate::api::WebResourceManager;
use crate::plugins::WebResourceProvider;

#[wrapper]
pub struct WebResourceProviders(RwLock<HashMap<String, Arc<dyn WebResourceProvider>>>);

pub struct WebResourceManagerImpl {
    web_resource_providers: WebResourceProviders,
}

#[component]
impl WebResourceManagerImpl {
    #[provides]
    fn new() -> Self {
        Self {
            web_resource_providers: WebResourceProviders(RwLock::new(HashMap::new())),
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
        self.web_resource_providers.0.read().unwrap().get(base_path.as_str()).cloned()
    }

    fn add_provider(&self, web_resource_provider: Arc<dyn WebResourceProvider>) {
        let base_path: String = web_resource_provider.get_base_path();
        debug!("Registering web resource provider with base path: {}", base_path);
        self.web_resource_providers.0.write().unwrap().insert(base_path, web_resource_provider.clone());
    }
}

impl Lifecycle for WebResourceManagerImpl {
    fn init(&self) {}

    fn post_init(&self) {}

    fn pre_shutdown(&self) {}

    fn shutdown(&self) {
        // TODO: remove?
        self.web_resource_providers.0.write().unwrap().clear();
    }
}
