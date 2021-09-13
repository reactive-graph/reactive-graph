use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::plugins::WebResourceProvider;

#[async_trait]
pub trait WebResourceManager: Send + Sync + Lifecycle {
    fn has(&self, web_resource_name: String) -> bool;
    fn get(&self, web_resource_name: String) -> Option<Arc<dyn WebResourceProvider>>;
    fn add_provider(&self, web_resource_provider: Arc<dyn WebResourceProvider>);
}
