use std::sync::Arc;

use async_trait::async_trait;
use reactive_graph_command_api::CommandTypeProvider;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_graph::Components;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentProviderRegistry;
#[allow(unused)]
use reactive_graph_type_system_api::TypeProvider as TypeProvider1;
use reactive_graph_type_system_api::TypeProvider;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct CommandComponentsProvider {}

#[derive(Component)]
pub struct CommandTypeProviderImpl {
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,
    // component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,
    component_provider: Arc<CommandComponentsProvider>,
}

#[async_trait]
#[component_alias]
impl CommandTypeProvider for CommandTypeProviderImpl {}

#[async_trait]
impl Lifecycle for CommandTypeProviderImpl {
    async fn init(&self) {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
    }

    async fn shutdown(&self) {
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
    }
}
