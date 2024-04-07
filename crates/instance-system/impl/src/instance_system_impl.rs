use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_instance_system_api::EntityInstanceImportExportManager;
use reactive_graph_instance_system_api::InstanceSystem;
use reactive_graph_instance_system_api::RelationInstanceImportExportManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_service_api::ReactiveSystem;

#[derive(Component)]
pub struct InstanceSystemImpl {
    entity_instance_import_export_manager: Arc<dyn EntityInstanceImportExportManager + Send + Sync>,
    relation_instance_import_export_manager: Arc<dyn RelationInstanceImportExportManager + Send + Sync>,

    reactive_system: Arc<dyn ReactiveSystem + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl InstanceSystem for InstanceSystemImpl {
    fn get_entity_instance_import_export_manager(&self) -> Arc<dyn EntityInstanceImportExportManager + Send + Sync> {
        self.entity_instance_import_export_manager.clone()
    }

    fn get_relation_instance_import_export_manager(&self) -> Arc<dyn RelationInstanceImportExportManager + Send + Sync> {
        self.relation_instance_import_export_manager.clone()
    }

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync> {
        self.reactive_system.clone()
    }
}

#[async_trait]
impl Lifecycle for InstanceSystemImpl {
    async fn init(&self) {
        self.entity_instance_import_export_manager.init().await;
        self.relation_instance_import_export_manager.init().await;
    }

    async fn post_init(&self) {
        self.entity_instance_import_export_manager.post_init().await;
        self.relation_instance_import_export_manager.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.relation_instance_import_export_manager.pre_shutdown().await;
        self.entity_instance_import_export_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.relation_instance_import_export_manager.shutdown().await;
        self.entity_instance_import_export_manager.shutdown().await;
    }
}
