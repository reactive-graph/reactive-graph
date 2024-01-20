use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use crate::TypeProvider;
use inexor_rgf_graph::FlowTypes;
use inexor_rgf_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait FlowTypeProviderRegistry: Send + Sync + Lifecycle {
    /// Registers a flow type provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<FlowTypes>>);

    /// Unregisters a flow type provider.
    async fn unregister_provider(&self, id: &str);
}
