use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_runtime_service_api::runtime_type_provider::RuntimeTypeProvider;
use reactive_graph_runtime_types::RuntimeComponentsProvider;
use reactive_graph_runtime_types::RuntimeEntityTypesProvider;
use reactive_graph_runtime_types::RuntimeFlowTypesProvider;
use reactive_graph_runtime_types::RuntimeRelationTypesProvider;
use reactive_graph_type_system_api::ComponentProviderRegistry;
use reactive_graph_type_system_api::EntityTypeProviderRegistry;
use reactive_graph_type_system_api::FlowTypeProviderRegistry;
use reactive_graph_type_system_api::RelationTypeProviderRegistry;
// #[allow(unused)]
// use reactive_graph_type_system_api::TypeProvider as TypeProvider2;
use reactive_graph_type_system_api::TypeProvider;

#[derive(Component)]
pub struct RuntimeTypeProviderImpl {
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,
    relation_type_provider_registry: Arc<dyn RelationTypeProviderRegistry + Send + Sync>,
    flow_type_provider_registry: Arc<dyn FlowTypeProviderRegistry + Send + Sync>,
    component_provider: Arc<RuntimeComponentsProvider>,
    entity_type_provider: Arc<RuntimeEntityTypesProvider>,
    relation_type_provider: Arc<RuntimeRelationTypesProvider>,
    flow_type_provider: Arc<RuntimeFlowTypesProvider>,
}

#[async_trait]
#[component_alias]
impl RuntimeTypeProvider for RuntimeTypeProviderImpl {}

#[async_trait]
impl Lifecycle for RuntimeTypeProviderImpl {
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
