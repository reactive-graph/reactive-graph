use std::sync::Arc;

use async_trait::async_trait;
use inexor_rgf_lifecycle::Lifecycle;
use springtime_di::injectable;

use crate::ComponentImportExportManager;
use crate::ComponentManager;
use crate::ComponentProviderRegistry;
use crate::EntityTypeImportExportManager;
use crate::EntityTypeManager;
use crate::EntityTypeProviderRegistry;
use crate::FlowTypeImportExportManager;
use crate::FlowTypeManager;
use crate::FlowTypeProviderRegistry;
use crate::NamespaceManager;
use crate::RelationTypeImportExportManager;
use crate::RelationTypeManager;
use crate::RelationTypeProviderRegistry;
use crate::TypeSystemEventManager;

#[injectable]
#[async_trait]
pub trait TypeSystem: Lifecycle {
    fn get_component_manager(&self) -> Arc<dyn ComponentManager + Send + Sync>;

    fn get_component_import_export_manager(&self) -> Arc<dyn ComponentImportExportManager + Send + Sync>;

    fn get_component_provider_registry(&self) -> Arc<dyn ComponentProviderRegistry + Send + Sync>;

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager + Send + Sync>;

    fn get_entity_type_import_export_manager(&self) -> Arc<dyn EntityTypeImportExportManager + Send + Sync>;

    fn get_entity_type_provider_registry(&self) -> Arc<dyn EntityTypeProviderRegistry + Send + Sync>;

    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager + Send + Sync>;

    fn get_flow_type_import_export_manager(&self) -> Arc<dyn FlowTypeImportExportManager + Send + Sync>;

    fn get_flow_type_provider_registry(&self) -> Arc<dyn FlowTypeProviderRegistry + Send + Sync>;

    fn get_namespace_manager(&self) -> Arc<dyn NamespaceManager + Send + Sync>;

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager + Send + Sync>;

    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager + Send + Sync>;

    fn get_relation_type_provider_registry(&self) -> Arc<dyn RelationTypeProviderRegistry + Send + Sync>;

    fn get_type_system_event_manager(&self) -> Arc<dyn TypeSystemEventManager + Send + Sync>;
}
