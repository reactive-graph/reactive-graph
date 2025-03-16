use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::NamespaceManager;
use reactive_graph_type_system_api::RelationTypeManager;

#[derive(Component)]
pub struct NamespaceManagerImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    flow_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
}
#[async_trait]
#[component_alias]
impl NamespaceManager for NamespaceManagerImpl {
    fn get_all(&self) -> HashSet<String> {
        self.component_manager
            .get_all()
            .iter()
            .map(|t| t.namespace())
            .chain(self.entity_type_manager.get_all().iter().map(|t| t.namespace()))
            .chain(self.relation_type_manager.get_all().iter().map(|t| t.namespace()))
            .chain(self.flow_type_manager.get_all().iter().map(|t| t.namespace()))
            .collect()
    }
}
