use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::plugins::WebResourceProvider;

#[async_trait]
pub trait WebResourceManager: Send + Sync + Lifecycle {
    /// Returns true, if a web resource provider exists with the given context path.
    fn has(&self, context_path: String) -> bool;

    /// Returns the web resource provider with the given context path.
    fn get(&self, context_path: String) -> Option<Arc<dyn WebResourceProvider>>;

    /// Returns the default web resource provider.
    fn get_default(&self) -> Option<Arc<dyn WebResourceProvider>>;

    /// Registers a web resource provider.
    async fn register_provider(&self, web_resource_provider: Arc<dyn WebResourceProvider>);

    /// Unregisters a web resource provider.
    async fn unregister_provider(&self, id: Uuid);
}
