use std::sync::Arc;

use async_trait::async_trait;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

use crate::ComponentImportExportManager;
use crate::ComponentManager;
use crate::EntityTypeImportExportManager;
use crate::EntityTypeManager;
use crate::FlowTypeImportExportManager;
use crate::FlowTypeManager;
use crate::NamespaceTreeManager;
use crate::NamespacedTypeManager;
use crate::RelationTypeImportExportManager;
use crate::RelationTypeManager;
use crate::TypeSystemEventManager;
use crate::TypeSystemProviderRegistry;

#[injectable]
#[async_trait]
pub trait TypeSystemSystem: Lifecycle {
    fn get_component_manager(&self) -> Arc<dyn ComponentManager + Send + Sync>;

    fn get_component_import_export_manager(&self) -> Arc<dyn ComponentImportExportManager + Send + Sync>;

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager + Send + Sync>;

    fn get_entity_type_import_export_manager(&self) -> Arc<dyn EntityTypeImportExportManager + Send + Sync>;

    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager + Send + Sync>;

    fn get_flow_type_import_export_manager(&self) -> Arc<dyn FlowTypeImportExportManager + Send + Sync>;

    fn get_namespace_tree_manager(&self) -> Arc<dyn NamespaceTreeManager + Send + Sync>;

    fn get_namespaced_type_manager(&self) -> Arc<dyn NamespacedTypeManager + Send + Sync>;

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager + Send + Sync>;

    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager + Send + Sync>;

    fn get_type_system_event_manager(&self) -> Arc<dyn TypeSystemEventManager + Send + Sync>;

    fn get_type_system_provider_registry(&self) -> Arc<dyn TypeSystemProviderRegistry + Send + Sync>;
}
