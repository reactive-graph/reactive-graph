use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentImportExportManager;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeImportExportManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeImportExportManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::NamespaceTreeManager;
use reactive_graph_type_system_api::NamespacedTypeManager;
use reactive_graph_type_system_api::RelationTypeImportExportManager;
use reactive_graph_type_system_api::RelationTypeManager;
use reactive_graph_type_system_api::TypeSystemEventManager;
use reactive_graph_type_system_api::TypeSystemProviderRegistry;
use reactive_graph_type_system_api::TypeSystemSystem;
use reactive_graph_type_system_api::TypeSystemTypeSystemRegistrator;

#[derive(Component)]
pub struct TypeSystemSystemImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    component_import_export_manager: Arc<dyn ComponentImportExportManager + Send + Sync>,
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    entity_type_import_export_manager: Arc<dyn EntityTypeImportExportManager + Send + Sync>,
    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
    flow_type_import_export_manager: Arc<dyn FlowTypeImportExportManager + Send + Sync>,
    namespaced_type_manager: Arc<dyn NamespacedTypeManager + Send + Sync>,
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    relation_type_import_export_manager: Arc<dyn RelationTypeImportExportManager + Send + Sync>,
    type_system_event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,
    type_system_provider_registry: Arc<dyn TypeSystemProviderRegistry + Send + Sync>,
    type_system_type_system_registrator: Arc<dyn TypeSystemTypeSystemRegistrator + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl TypeSystemSystem for TypeSystemSystemImpl {
    fn get_component_manager(&self) -> Arc<dyn ComponentManager + Send + Sync> {
        self.component_manager.clone()
    }

    fn get_component_import_export_manager(&self) -> Arc<dyn ComponentImportExportManager + Send + Sync> {
        self.component_import_export_manager.clone()
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager + Send + Sync> {
        self.entity_type_manager.clone()
    }

    fn get_entity_type_import_export_manager(&self) -> Arc<dyn EntityTypeImportExportManager + Send + Sync> {
        self.entity_type_import_export_manager.clone()
    }

    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager + Send + Sync> {
        self.flow_type_manager.clone()
    }

    fn get_flow_type_import_export_manager(&self) -> Arc<dyn FlowTypeImportExportManager + Send + Sync> {
        self.flow_type_import_export_manager.clone()
    }

    fn get_namespace_tree_manager(&self) -> Arc<dyn NamespaceTreeManager + Send + Sync> {
        todo!()
    }

    fn get_namespaced_type_manager(&self) -> Arc<dyn NamespacedTypeManager + Send + Sync> {
        self.namespaced_type_manager.clone()
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager + Send + Sync> {
        self.relation_type_manager.clone()
    }

    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager + Send + Sync> {
        self.relation_type_import_export_manager.clone()
    }

    fn get_type_system_event_manager(&self) -> Arc<dyn TypeSystemEventManager + Send + Sync> {
        self.type_system_event_manager.clone()
    }

    fn get_type_system_provider_registry(&self) -> Arc<dyn TypeSystemProviderRegistry + Send + Sync> {
        self.type_system_provider_registry.clone()
    }
}

#[async_trait]
impl Lifecycle for TypeSystemSystemImpl {
    async fn init(&self) {
        // Type System
        self.component_manager.init().await;
        self.entity_type_manager.init().await;
        self.relation_type_manager.init().await;
        self.flow_type_manager.init().await;
        // Type Providers
        self.type_system_provider_registry.init().await;
        // Import / Export
        self.component_import_export_manager.init().await;
        self.entity_type_import_export_manager.init().await;
        self.relation_type_import_export_manager.init().await;
        self.flow_type_import_export_manager.init().await;
        // Event System
        self.type_system_event_manager.init().await;
        // Register own type system
        self.type_system_type_system_registrator.init().await;
    }

    async fn post_init(&self) {
        // Type System
        self.component_manager.post_init().await;
        self.entity_type_manager.post_init().await;
        self.relation_type_manager.post_init().await;
        self.flow_type_manager.post_init().await;
        // Type Providers
        self.type_system_provider_registry.post_init().await;
        // Event System
        self.type_system_event_manager.post_init().await;
        // Register own type system
        self.type_system_type_system_registrator.post_init().await;
    }

    async fn pre_shutdown(&self) {
        // Unregister own type system
        self.type_system_type_system_registrator.pre_shutdown().await;
        // Event System
        self.type_system_event_manager.pre_shutdown().await;
        // Type Providers
        self.type_system_provider_registry.pre_shutdown().await;
        // Type System
        self.flow_type_manager.pre_shutdown().await;
        self.relation_type_manager.pre_shutdown().await;
        self.entity_type_manager.pre_shutdown().await;
        self.component_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        // Unregister own type system
        self.type_system_type_system_registrator.shutdown().await;
        // Event System
        self.type_system_event_manager.shutdown().await;
        // Type Providers
        self.type_system_provider_registry.shutdown().await;
        // Type System
        self.flow_type_manager.shutdown().await;
        self.relation_type_manager.shutdown().await;
        self.entity_type_manager.shutdown().await;
        self.component_manager.shutdown().await;
    }
}
