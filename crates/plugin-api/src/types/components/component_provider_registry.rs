use std::sync::Arc;

use async_trait::async_trait;

use crate::model::Components;
use crate::TypeProvider;

#[async_trait]
pub trait ComponentProviderRegistry: Send + Sync {
    /// Registers a component provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<Components>>);

    /// Unregisters a component provider.
    async fn unregister_provider(&self, id: &str);
}
