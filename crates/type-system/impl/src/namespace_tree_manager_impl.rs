use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

use reactive_graph_graph::Namespaces;
use reactive_graph_type_system_api::NamespaceTreeManager;
use reactive_graph_type_system_api::NamespacedTypeManager;

#[derive(Component)]
pub struct NamespaceTreeManagerImpl {
    namespaced_type_manager: Arc<dyn NamespacedTypeManager + Send + Sync>,
    // #[component(default = "Namespace::new")]
    // namespaces: Namespaces,
}

#[async_trait]
#[component_alias]
impl NamespaceTreeManager for NamespaceTreeManagerImpl {
    fn get_all(&self) -> Namespaces {
        self.namespaced_type_manager.get_all().get_all_parent_paths_recursively()
    }
}
