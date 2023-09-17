use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use inexor_rgf_plugin_api::WebResourceManager;
use inexor_rgf_plugin_api::WebResourceProvider;

pub struct WebResourceManagerImpl {
    web_resource_manager: Arc<dyn crate::api::WebResourceManager>,
}

impl WebResourceManagerImpl {
    pub fn new(web_resource_manager: Arc<dyn crate::api::WebResourceManager>) -> Self {
        Self { web_resource_manager }
    }
}

#[async_trait]
impl WebResourceManager for WebResourceManagerImpl {
    async fn register_provider(&self, web_resource_provider: Arc<dyn WebResourceProvider>) {
        self.web_resource_manager.register_provider(web_resource_provider).await;
    }

    async fn unregister_provider(&self, id: Uuid) {
        self.web_resource_manager.unregister_provider(id).await;
    }
}
