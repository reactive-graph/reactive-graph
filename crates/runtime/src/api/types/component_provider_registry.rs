use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::Components;
use crate::plugins::TypeProvider;

#[async_trait]
pub trait ComponentProviderRegistry: Send + Sync + Lifecycle {
    /// Registers a component provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<Components>>);

    /// Unregisters a component provider.
    async fn unregister_provider(&self, id: &str);
}
