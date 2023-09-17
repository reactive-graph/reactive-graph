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

#[macro_export]
macro_rules! register_flow_type_provider {
    ($context: expr, $provider: expr) => {
        $context.get_flow_type_provider_registry().register_provider($provider.clone()).await;
    };
}

#[macro_export]
macro_rules! unregister_flow_type_provider {
    ($context: expr, $id: expr) => {
        $context.get_flow_type_provider_registry().unregister_provider(&$id).await;
    };
}
