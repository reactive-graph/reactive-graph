use std::sync::Arc;

use async_trait::async_trait;

use crate::model::FlowTypes;
use crate::TypeProvider;

#[async_trait]
pub trait FlowTypeProviderRegistry: Send + Sync {
    /// Registers a flow type provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<FlowTypes>>);

    /// Unregisters a flow type provider.
    async fn unregister_provider(&self, id: &str);
}
