use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_service_api::ReactiveSystem;

use crate::EntityInstanceImportExportManager;
use crate::RelationInstanceImportExportManager;

#[injectable]
#[async_trait]
pub trait InstanceSystem: Lifecycle {
    fn get_entity_instance_import_export_manager(&self) -> Arc<dyn EntityInstanceImportExportManager + Send + Sync>;

    fn get_relation_instance_import_export_manager(&self) -> Arc<dyn RelationInstanceImportExportManager + Send + Sync>;

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync>;
}
