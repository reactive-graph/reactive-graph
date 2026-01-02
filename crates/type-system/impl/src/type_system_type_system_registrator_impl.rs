use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use reactive_graph_graph::TypeSystemProvider;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::TypeSystemProviderRegistry;
use reactive_graph_type_system_api::TypeSystemTypeSystemRegistrator;
// use reactive_graph_type_system_model::type_system::TYPE_SYSTEM;
// use reactive_graph_type_system_model::type_system::TYPE_SYSTEM_NAMESPACE;
use springtime_di::Component;
use springtime_di::component_alias;

#[derive(Component)]
pub struct TypeSystemTypeSystemRegistratorImpl {
    type_system_provider_registry: Arc<dyn TypeSystemProviderRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl TypeSystemTypeSystemRegistrator for TypeSystemTypeSystemRegistratorImpl {}

#[async_trait]
impl Lifecycle for TypeSystemTypeSystemRegistratorImpl {
    async fn init(&self) {
        self.type_system_provider_registry
            .register_provider(TypeSystemProvider::new(
                reactive_graph_model_core::type_system::TYPE_SYSTEM_NAMESPACE.clone(),
                reactive_graph_model_core::type_system::TYPE_SYSTEM.clone(),
            ))
            .await;
        self.type_system_provider_registry
            .register_provider(TypeSystemProvider::new(
                reactive_graph_model_meta::type_system::TYPE_SYSTEM_NAMESPACE.clone(),
                reactive_graph_model_meta::type_system::TYPE_SYSTEM.clone(),
            ))
            .await;
        self.type_system_provider_registry
            .register_provider(TypeSystemProvider::new(
                reactive_graph_type_system_model::type_system::TYPE_SYSTEM_NAMESPACE.clone(),
                reactive_graph_type_system_model::type_system::TYPE_SYSTEM.clone(),
            ))
            .await;
    }

    async fn shutdown(&self) {
        self.type_system_provider_registry
            .unregister_provider(reactive_graph_type_system_model::type_system::TYPE_SYSTEM_NAMESPACE.deref())
            .await;
        self.type_system_provider_registry
            .unregister_provider(reactive_graph_model_meta::type_system::TYPE_SYSTEM_NAMESPACE.deref())
            .await;
        self.type_system_provider_registry
            .unregister_provider(reactive_graph_model_core::type_system::TYPE_SYSTEM_NAMESPACE.deref())
            .await;
    }
}
