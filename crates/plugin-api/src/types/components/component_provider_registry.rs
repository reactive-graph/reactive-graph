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

#[macro_export]
macro_rules! register_component_provider {
    ($context: expr, $provider: expr) => {
        // $crate::get_context!($context, inexor_rgf_plugin_api::PluginActivationError::PluginRequiresMissingPluginContext)
        $context.get_component_provider_registry().register_provider($provider.clone()).await;
    };
}

#[macro_export]
macro_rules! unregister_component_provider {
    ($context: expr, $id: expr) => {
        // $crate::get_context!($context, inexor_rgf_plugin_api::PluginDeactivationError::PluginRequiresMissingPluginContext)
        $context.get_component_provider_registry().unregister_provider(&$id).await;
    };
}

// #[macro_export]
// macro_rules! register_type_providers {
//     (
//         $context: expr
//         $(, component_provider $component_provider: expr)*
//         $(, entity_type_provider $entity_type_provider: expr)*
//         $(, relation_type_provider $relation_type_provider: expr)*
//         $(, flow_type_provider $flow_type_provider: expr)*
//         $(,)?
//     ) => {
//         $(
//             $context
//                 .clone()
//                 .ok_or(PluginActivationError::PluginRequiresMissingPluginContext)?
//                 .get_component_provider_registry()
//                 .register_provider($component_provider.clone())
//                 .await;
//         )*
//         $(
//             $context
//                 .clone()
//                 .ok_or(PluginActivationError::PluginRequiresMissingPluginContext)?
//                 .get_entity_type_provider_registry()
//                 .register_provider($entity_type_provider.clone())
//                 .await;
//         )*
//         $(
//             $context
//                 .clone()
//                 .ok_or(PluginActivationError::PluginRequiresMissingPluginContext)?
//                 .get_relation_type_provider_registry()
//                 .register_provider($relation_type_provider.clone())
//                 .await;
//         )*
//         $(
//             $context
//                 .clone()
//                 .ok_or(PluginActivationError::PluginRequiresMissingPluginContext)?
//                 .get_flow_type_provider_registry()
//                 .register_provider($relation_type_provider.clone())
//                 .await;
//         )*
//     };
// }

// #[inline]
// pub async fn register_component_provider(
//     context: Option<Arc<dyn PluginContext + Send + Sync>>,
//     component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,
// ) -> Result<(), PluginActivationError> {
//     context
//         .clone()
//         .ok_or(PluginActivationError::PluginRequiresMissingPluginContext)?
//         .get_component_provider_registry()
//         .register_provider(component_provider.clone())
//         .await;
//     Ok(())
// }
//
// pub async fn register_type_provider<T: crate::model::NamespacedTypeContainer>(
//     context: &Option<Arc<dyn PluginContext + Send + Sync>>,
//     type_provider: Arc<dyn TypeProvider<T> + Send + Sync>,
// ) -> Result<(), PluginActivationError> {
//     context
//         .clone()
//         .ok_or(PluginActivationError::PluginRequiresMissingPluginContext)?
//         .get_types_provider_registry::<T>()
//         .register_provider(type_provider.clone())
//         .await;
//     Ok(())
// }
