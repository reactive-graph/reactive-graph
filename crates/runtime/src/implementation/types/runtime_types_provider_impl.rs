use async_trait::async_trait;

use crate::plugins::TypeProvider;

use crate::api::ComponentProviderRegistry;
use crate::api::EntityTypeProviderRegistry;
use crate::api::FlowTypeProviderRegistry;
use crate::api::Lifecycle;
use crate::api::RelationTypeProviderRegistry;
use crate::api::RuntimeTypesProvider;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;
#[allow(unused)]
use crate::plugins::TypeProvider as TypeProvider1;

#[component]
#[derive(TypeProvider)]
#[type_provider(tys = "crate::model::Components", path = "types/components", component_alias = false)]
pub struct CoreComponentsProvider {}

#[component]
#[derive(TypeProvider)]
#[type_provider(tys = "crate::model::EntityTypes", path = "types/entities", component_alias = false)]
pub struct CoreEntityTypesProvider {}

#[component]
#[derive(TypeProvider)]
#[type_provider(tys = "crate::model::RelationTypes", path = "types/relations", component_alias = false)]
pub struct CoreRelationTypesProvider {}

#[component]
#[derive(TypeProvider)]
#[type_provider(tys = "crate::model::FlowTypes", path = "types/flows", component_alias = false)]
pub struct CoreFlowTypesProvider {}

#[component]
pub struct RuntimeTypesProviderImpl {
    component_provider_registry: Wrc<dyn ComponentProviderRegistry>,
    entity_type_provider_registry: Wrc<dyn EntityTypeProviderRegistry>,
    relation_type_provider_registry: Wrc<dyn RelationTypeProviderRegistry>,
    flow_type_provider_registry: Wrc<dyn FlowTypeProviderRegistry>,
    component_provider: Wrc<CoreComponentsProvider>,
    entity_type_provider: Wrc<CoreEntityTypesProvider>,
    relation_type_provider: Wrc<CoreRelationTypesProvider>,
    flow_type_provider: Wrc<CoreFlowTypesProvider>,
}

#[async_trait]
#[provides]
impl RuntimeTypesProvider for RuntimeTypesProviderImpl {}

#[async_trait]
#[provides]
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
        self.flow_type_provider_registry.unregister_provider(&self.flow_type_provider.id()).await;
        self.relation_type_provider_registry
            .unregister_provider(&self.relation_type_provider.id())
            .await;
        self.entity_type_provider_registry.unregister_provider(&self.entity_type_provider.id()).await;
        self.component_provider_registry.unregister_provider(&self.component_provider.id()).await;
    }
}
