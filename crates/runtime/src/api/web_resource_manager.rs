use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::plugins::WebResourceProvider;

#[async_trait]
pub trait WebResourceManager: Send + Sync + Lifecycle {
    fn has(&self, base_path: String) -> bool;
    fn get(&self, base_path: String) -> Option<Arc<dyn WebResourceProvider>>;
    fn get_default(&self) -> Option<Arc<dyn WebResourceProvider>>;
    fn get_default_base_path(&self) -> Option<String>;

    /// Registers a web resource provider.
    fn add_provider(&self, id: Uuid, web_resource_provider: Arc<dyn WebResourceProvider>);

    /// Unregisters a web resource provider.
    fn remove_provider(&self, id: &Uuid);
}
