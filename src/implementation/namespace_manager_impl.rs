use crate::model::NamespacedTypeGetter;
use async_trait::async_trait;
use std::collections::HashSet;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::NamespaceManager;
use crate::api::RelationTypeManager;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;

#[component]
pub struct NamespaceManagerImpl {
    component_manager: Wrc<dyn ComponentManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
}
#[async_trait]
#[provides]
impl NamespaceManager for NamespaceManagerImpl {
    fn get_all(&self) -> HashSet<String> {
        self.component_manager
            .get_all()
            .iter()
            .map(|t| t.namespace())
            .chain(self.entity_type_manager.get_all().iter().map(|t| t.namespace()))
            .chain(self.relation_type_manager.get_all().iter().map(|t| t.namespace()))
            .collect()
    }
}
