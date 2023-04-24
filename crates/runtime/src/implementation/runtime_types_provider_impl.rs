use async_trait::async_trait;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::Lifecycle;
use crate::api::RelationTypeManager;
use crate::api::RuntimeTypesProvider;
use crate::di::*;

crate::plugins::component_provider_impl!(Core, "types/components");
crate::plugins::entity_type_provider_impl!(Core, "types/entities");
crate::plugins::relation_type_provider_impl!(Core, "types/relations");
crate::plugins::flow_type_provider_impl!(Core, "types/flows");

#[component]
pub struct RuntimeTypesProviderImpl {
    component_manager: Wrc<dyn ComponentManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    flow_type_manager: Wrc<dyn FlowTypeManager>,
    component_provider: Wrc<CoreComponentProviderImpl>,
    entity_type_provider: Wrc<CoreEntityTypeProviderImpl>,
    relation_type_provider: Wrc<CoreRelationTypeProviderImpl>,
    flow_type_provider: Wrc<CoreFlowTypeProviderImpl>,
}

#[async_trait]
#[provides]
impl RuntimeTypesProvider for RuntimeTypesProviderImpl {}

#[async_trait]
impl Lifecycle for RuntimeTypesProviderImpl {
    async fn init(&self) {
        self.component_manager.add_provider(self.component_provider.clone());
        self.entity_type_manager.add_provider(self.entity_type_provider.clone());
        self.relation_type_manager.add_provider(self.relation_type_provider.clone());
        self.flow_type_manager.add_provider(self.flow_type_provider.clone());
    }
}
