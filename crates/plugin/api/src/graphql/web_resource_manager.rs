use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::WebResourceProvider;

#[async_trait]
pub trait WebResourceManager: Send + Sync {
    /// Registers a web resource provider.
    async fn register_provider(&self, web_resource_provider: Arc<dyn WebResourceProvider>);

    /// Unregisters a web resource provider.
    async fn unregister_provider(&self, id: Uuid);
}
