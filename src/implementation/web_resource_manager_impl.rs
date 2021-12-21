use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use log::debug;
use waiter_di::{component, provides, wrapper};

use crate::api::Lifecycle;
use crate::api::WebResourceManager;
use crate::plugins::WebResourceProvider;

#[wrapper]
pub struct WebResourceProviders(RwLock<std::collections::HashMap<String, Arc<dyn WebResourceProvider>>>);

pub struct WebResourceManagerImpl {
    web_resource_providers: WebResourceProviders,
}

#[component]
impl WebResourceManagerImpl {
    #[provides]
    fn new() -> Self {
        Self {
            web_resource_providers: WebResourceProviders(RwLock::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait]
#[provides]
impl WebResourceManager for WebResourceManagerImpl {
    fn has(&self, web_resource_name: String) -> bool {
        self.get(web_resource_name).is_some()
    }

    fn get(&self, web_resource_name: String) -> Option<Arc<dyn WebResourceProvider>> {
        self.web_resource_providers
            .0
            .read()
            .unwrap()
            .get(web_resource_name.as_str())
            .map(|web_resource_provider| web_resource_provider.clone())
        // let x =
        // match x {
        //     Some(web_resource_provider) => Some(web_resource_provider.clone()),
        //     None => None
        // }

        // .to_vec()
        // .into_iter()
        // .find(|web_resource| web_resource.name == web_resource_name)
    }

    fn add_provider(&self, web_resource_provider: Arc<dyn WebResourceProvider>) {
        let name: String = web_resource_provider.get_name();
        debug!("Registering web resource provider: {}", name.clone());
        self.web_resource_providers
            .0
            .write()
            .unwrap()
            .insert(name.clone(), web_resource_provider.clone());
    }
}

impl Lifecycle for WebResourceManagerImpl {
    fn init(&self) {}

    fn shutdown(&self) {
        // TODO: remove?
        self.web_resource_providers.0.write().unwrap().clear();
    }
}
