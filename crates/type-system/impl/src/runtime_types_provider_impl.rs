use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::Components;
use reactive_graph_graph::EntityTypes;
use reactive_graph_graph::FlowTypes;
use reactive_graph_graph::RelationTypes;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentProviderRegistry;
use reactive_graph_type_system_api::EntityTypeProviderRegistry;
use reactive_graph_type_system_api::FlowTypeProviderRegistry;
use reactive_graph_type_system_api::RelationTypeProviderRegistry;
use reactive_graph_type_system_api::RuntimeTypesProvider;
#[allow(unused)]
use reactive_graph_type_system_api::TypeProvider as TypeProvider1;
use reactive_graph_type_system_api::TypeProvider;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct CoreComponentsProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "EntityTypes", path = "types/entities")]
pub struct CoreEntityTypesProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "RelationTypes", path = "types/relations")]
pub struct CoreRelationTypesProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "FlowTypes", path = "types/flows")]
pub struct CoreFlowTypesProvider {}

#[derive(Component)]
pub struct RuntimeTypesProviderImpl {
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,
    relation_type_provider_registry: Arc<dyn RelationTypeProviderRegistry + Send + Sync>,
    flow_type_provider_registry: Arc<dyn FlowTypeProviderRegistry + Send + Sync>,
    component_provider: Arc<CoreComponentsProvider>,
    // component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,
    entity_type_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,
    relation_type_provider: Arc<dyn TypeProvider<RelationTypes> + Send + Sync>,
    flow_type_provider: Arc<dyn TypeProvider<FlowTypes> + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RuntimeTypesProvider for RuntimeTypesProviderImpl {}

#[async_trait]
impl Lifecycle for RuntimeTypesProviderImpl {
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
