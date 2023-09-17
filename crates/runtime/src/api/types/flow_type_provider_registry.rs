use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::FlowTypes;
use crate::plugins::TypeProvider;

#[async_trait]
pub trait FlowTypeProviderRegistry: Send + Sync + Lifecycle {
    /// Registers a flow type provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<FlowTypes>>);

    /// Unregisters a flow type provider.
    async fn unregister_provider(&self, id: &str);
}
