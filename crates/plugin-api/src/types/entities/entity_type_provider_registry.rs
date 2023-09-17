use std::sync::Arc;

use async_trait::async_trait;

use crate::model::EntityTypes;
use crate::TypeProvider;

#[async_trait]
pub trait EntityTypeProviderRegistry: Send + Sync {
    /// Registers an entity type provider.
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<EntityTypes>>);

    /// Unregisters an entity type provider.
    async fn unregister_provider(&self, id: &str);
}

#[macro_export]
macro_rules! register_entity_type_provider {
    ($context: expr, $provider: expr) => {
        // $crate::get_context!($context, inexor_rgf_plugin_api::PluginActivationError::PluginRequiresMissingPluginContext)
        $context.get_entity_type_provider_registry().register_provider($provider.clone()).await;
    };
}

#[macro_export]
macro_rules! unregister_entity_type_provider {
    ($context: expr, $id: expr) => {
        // $crate::get_context!($context, inexor_rgf_plugin_api::PluginDeactivationError::PluginRequiresMissingPluginContext)
        $context.get_entity_type_provider_registry().unregister_provider(&$id).await;
    };
}
