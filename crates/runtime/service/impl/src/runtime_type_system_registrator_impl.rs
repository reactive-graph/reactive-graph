use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use reactive_graph_graph::TypeSystemProvider;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_runtime_model::type_system::TYPE_SYSTEM;
use reactive_graph_runtime_model::type_system::TYPE_SYSTEM_NAMESPACE;
use reactive_graph_runtime_service_api::RuntimeTypeSystemRegistrator;
use reactive_graph_type_system_api::TypeSystemProviderRegistry;
use springtime_di::Component;
use springtime_di::component_alias;

#[derive(Component)]
pub struct RuntimeTypeSystemRegistratorImpl {
    type_system_provider_registry: Arc<dyn TypeSystemProviderRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RuntimeTypeSystemRegistrator for RuntimeTypeSystemRegistratorImpl {}

#[async_trait]
impl Lifecycle for RuntimeTypeSystemRegistratorImpl {
    async fn init(&self) {
        self.type_system_provider_registry
            .register_provider(TypeSystemProvider::new(TYPE_SYSTEM_NAMESPACE.clone(), TYPE_SYSTEM.clone()))
            .await;
    }

    async fn shutdown(&self) {
        self.type_system_provider_registry.unregister_provider(TYPE_SYSTEM_NAMESPACE.deref()).await;
    }
}
