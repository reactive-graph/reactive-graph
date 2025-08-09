use std::sync::Arc;

use async_trait::async_trait;
use reactive_graph_command_api::CommandTypeProvider;
use reactive_graph_command_types::CommandComponentsProvider;
use reactive_graph_command_types::CommandEntityTypesProvider;
use reactive_graph_command_types::CommandFlowTypesProvider;
use reactive_graph_command_types::CommandRelationTypesProvider;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentProviderRegistry;
use reactive_graph_type_system_api::EntityTypeProviderRegistry;
use reactive_graph_type_system_api::FlowTypeProviderRegistry;
use reactive_graph_type_system_api::RelationTypeProviderRegistry;
use reactive_graph_type_system_api::TypeProvider;
use springtime_di::Component;
use springtime_di::component_alias;

#[derive(Component)]
pub struct CommandTypeProviderImpl {
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,
    relation_type_provider_registry: Arc<dyn RelationTypeProviderRegistry + Send + Sync>,
    flow_type_provider_registry: Arc<dyn FlowTypeProviderRegistry + Send + Sync>,
    component_provider: Arc<CommandComponentsProvider>,
    entity_type_provider: Arc<CommandEntityTypesProvider>,
    relation_type_provider: Arc<CommandRelationTypesProvider>,
    flow_type_provider: Arc<CommandFlowTypesProvider>,
}

#[async_trait]
#[component_alias]
impl CommandTypeProvider for CommandTypeProviderImpl {}

#[async_trait]
impl Lifecycle for CommandTypeProviderImpl {
    async fn init(&self) {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_type_provider.clone()).await;
        self.relation_type_provider_registry
            .register_provider(self.relation_type_provider.clone())
            .await;
        self.flow_type_provider_registry.register_provider(self.flow_type_provider.clone()).await;
    }

    async fn shutdown(&self) {
        self.flow_type_provider_registry.unregister_provider(self.flow_type_provider.id()).await;
        self.relation_type_provider_registry.unregister_provider(self.relation_type_provider.id()).await;
        self.entity_type_provider_registry.unregister_provider(self.entity_type_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
    }
}
