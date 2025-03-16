use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentImportExportManager;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::ComponentProviderRegistry;
use reactive_graph_type_system_api::EntityTypeImportExportManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::EntityTypeProviderRegistry;
use reactive_graph_type_system_api::FlowTypeImportExportManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::FlowTypeProviderRegistry;
use reactive_graph_type_system_api::NamespaceManager;
use reactive_graph_type_system_api::RelationTypeImportExportManager;
use reactive_graph_type_system_api::RelationTypeManager;
use reactive_graph_type_system_api::RelationTypeProviderRegistry;
use reactive_graph_type_system_api::RuntimeTypesProvider;
use reactive_graph_type_system_api::TypeSystem;
use reactive_graph_type_system_api::TypeSystemEventManager;

#[derive(Component)]
pub struct TypeSystemImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    component_import_export_manager: Arc<dyn ComponentImportExportManager + Send + Sync>,
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    entity_type_import_export_manager: Arc<dyn EntityTypeImportExportManager + Send + Sync>,
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,
    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
    flow_type_import_export_manager: Arc<dyn FlowTypeImportExportManager + Send + Sync>,
    flow_type_provider_registry: Arc<dyn FlowTypeProviderRegistry + Send + Sync>,
    namespace_manager: Arc<dyn NamespaceManager + Send + Sync>,
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    relation_type_import_export_manager: Arc<dyn RelationTypeImportExportManager + Send + Sync>,
    relation_type_provider_registry: Arc<dyn RelationTypeProviderRegistry + Send + Sync>,
    type_system_event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,
    // TODO: move it out of the type system else the type system cannot be empty!
    runtime_types_provider: Arc<dyn RuntimeTypesProvider + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl TypeSystem for TypeSystemImpl {
    fn get_component_manager(&self) -> Arc<dyn ComponentManager + Send + Sync> {
        self.component_manager.clone()
    }

    fn get_component_import_export_manager(&self) -> Arc<dyn ComponentImportExportManager + Send + Sync> {
        self.component_import_export_manager.clone()
    }

    fn get_component_provider_registry(&self) -> Arc<dyn ComponentProviderRegistry + Send + Sync> {
        self.component_provider_registry.clone()
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager + Send + Sync> {
        self.entity_type_manager.clone()
    }

    fn get_entity_type_import_export_manager(&self) -> Arc<dyn EntityTypeImportExportManager + Send + Sync> {
        self.entity_type_import_export_manager.clone()
    }

    fn get_entity_type_provider_registry(&self) -> Arc<dyn EntityTypeProviderRegistry + Send + Sync> {
        self.entity_type_provider_registry.clone()
    }

    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager + Send + Sync> {
        self.flow_type_manager.clone()
    }

    fn get_flow_type_import_export_manager(&self) -> Arc<dyn FlowTypeImportExportManager + Send + Sync> {
        self.flow_type_import_export_manager.clone()
    }

    fn get_flow_type_provider_registry(&self) -> Arc<dyn FlowTypeProviderRegistry + Send + Sync> {
        self.flow_type_provider_registry.clone()
    }

    fn get_namespace_manager(&self) -> Arc<dyn NamespaceManager + Send + Sync> {
        self.namespace_manager.clone()
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager + Send + Sync> {
        self.relation_type_manager.clone()
    }

    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager + Send + Sync> {
        self.relation_type_import_export_manager.clone()
    }

    fn get_relation_type_provider_registry(&self) -> Arc<dyn RelationTypeProviderRegistry + Send + Sync> {
        self.relation_type_provider_registry.clone()
    }

    fn get_type_system_event_manager(&self) -> Arc<dyn TypeSystemEventManager + Send + Sync> {
        self.type_system_event_manager.clone()
    }
}

#[async_trait]
impl Lifecycle for TypeSystemImpl {
    async fn init(&self) {
        // Type System
        self.component_manager.init().await;
        self.entity_type_manager.init().await;
        self.relation_type_manager.init().await;
        self.flow_type_manager.init().await;
        // Type Providers
        self.component_provider_registry.init().await;
        self.entity_type_provider_registry.init().await;
        self.relation_type_provider_registry.init().await;
        self.flow_type_provider_registry.init().await;
        // Import / Export
        self.component_import_export_manager.init().await;
        self.entity_type_import_export_manager.init().await;
        self.relation_type_import_export_manager.init().await;
        self.flow_type_import_export_manager.init().await;
        // Event System
        self.type_system_event_manager.init().await;
        // Essential types
        // TODO: move it out of the type system else the type system cannot be empty!
        self.runtime_types_provider.init().await;
    }

    async fn post_init(&self) {
        // Type System
        self.component_manager.post_init().await;
        self.entity_type_manager.post_init().await;
        self.relation_type_manager.post_init().await;
        self.flow_type_manager.post_init().await;
        // Type Providers
        self.component_provider_registry.post_init().await;
        self.entity_type_provider_registry.post_init().await;
        self.relation_type_provider_registry.post_init().await;
        self.flow_type_provider_registry.post_init().await;
        // Event System
        self.type_system_event_manager.post_init().await;
        // Essential types
        // TODO: move it out of the type system else the type system cannot be empty!
        self.runtime_types_provider.post_init().await;
    }

    async fn pre_shutdown(&self) {
        // Essential types
        // TODO: move it out of the type system else the type system cannot be empty!
        self.runtime_types_provider.pre_shutdown().await;
        // Event System
        self.type_system_event_manager.pre_shutdown().await;
        // Type Providers
        self.flow_type_provider_registry.pre_shutdown().await;
        self.relation_type_provider_registry.pre_shutdown().await;
        self.entity_type_provider_registry.pre_shutdown().await;
        self.component_provider_registry.pre_shutdown().await;
        // Type System
        self.flow_type_manager.pre_shutdown().await;
        self.relation_type_manager.pre_shutdown().await;
        self.entity_type_manager.pre_shutdown().await;
        self.component_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        // Essential types
        // TODO: move it out of the type system else the type system cannot be empty!
        self.runtime_types_provider.shutdown().await;
        // Event System
        self.type_system_event_manager.shutdown().await;
        // Type Providers
        self.flow_type_provider_registry.shutdown().await;
        self.relation_type_provider_registry.shutdown().await;
        self.entity_type_provider_registry.shutdown().await;
        self.component_provider_registry.shutdown().await;
        // Type System
        self.flow_type_manager.shutdown().await;
        self.relation_type_manager.shutdown().await;
        self.entity_type_manager.shutdown().await;
        self.component_manager.shutdown().await;
    }
}
