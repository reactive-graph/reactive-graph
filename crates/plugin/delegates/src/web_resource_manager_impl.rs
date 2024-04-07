use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use reactive_graph_plugin_api::WebResourceProvider;

pub struct WebResourceManagerDelegate {
    web_resource_manager: Arc<dyn reactive_graph_runtime_web_api::WebResourceManager + Send + Sync>,
}

impl WebResourceManagerDelegate {
    pub fn new(web_resource_manager: Arc<dyn reactive_graph_runtime_web_api::WebResourceManager + Send + Sync>) -> Self {
        Self { web_resource_manager }
    }
}

#[async_trait]
impl reactive_graph_plugin_api::WebResourceManager for WebResourceManagerDelegate {
    async fn register_provider(&self, web_resource_provider: Arc<dyn WebResourceProvider>) {
        self.web_resource_manager.register_provider(web_resource_provider).await;
    }

    async fn unregister_provider(&self, id: Uuid) {
        self.web_resource_manager.unregister_provider(id).await;
    }
}
