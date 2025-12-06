use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use reactive_graph_command_api::CommandTypeSystemRegistrator;
use reactive_graph_command_model::type_system::TYPE_SYSTEM;
use reactive_graph_command_model::type_system::TYPE_SYSTEM_NAMESPACE;
use reactive_graph_graph::TypeSystemProvider;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::TypeSystemProviderRegistry;
use springtime_di::Component;
use springtime_di::component_alias;

pub static TYPE_SYSTEM_ID: &str = "reactive_graph::runtime";

#[derive(Component)]
pub struct CommandTypeSystemRegistratorImpl {
    type_system_provider_registry: Arc<dyn TypeSystemProviderRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl CommandTypeSystemRegistrator for CommandTypeSystemRegistratorImpl {}

#[async_trait]
impl Lifecycle for CommandTypeSystemRegistratorImpl {
    async fn init(&self) {
        self.type_system_provider_registry
            .register_provider(TypeSystemProvider::new(TYPE_SYSTEM_NAMESPACE.clone(), TYPE_SYSTEM.clone()))
            .await;
    }

    async fn shutdown(&self) {
        self.type_system_provider_registry.unregister_provider(TYPE_SYSTEM_NAMESPACE.deref()).await;
    }
}
