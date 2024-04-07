use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_behaviour_service_api::EntityBehaviourManager;
use reactive_graph_behaviour_service_api::EntityBehaviourRegistry;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourManager;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourRegistry;
use reactive_graph_behaviour_service_api::RelationBehaviourManager;
use reactive_graph_behaviour_service_api::RelationBehaviourRegistry;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourManager;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourRegistry;
use reactive_graph_command_api::CommandManager;
use reactive_graph_config_api::ConfigManager;
use reactive_graph_graphql_api::GraphQLQueryService;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_api::PluginContext;
use reactive_graph_plugin_delegates::CommandManagerDelegate;
use reactive_graph_plugin_delegates::ComponentImportExportManagerDelegate;
use reactive_graph_plugin_delegates::ComponentManagerDelegate;
use reactive_graph_plugin_delegates::ComponentProviderRegistryDelegate;
use reactive_graph_plugin_delegates::ConfigManagerDelegate;
use reactive_graph_plugin_delegates::EntityBehaviourRegistryDelegate;
use reactive_graph_plugin_delegates::EntityComponentBehaviourRegistryDelegate;
use reactive_graph_plugin_delegates::EntityInstanceManagerDelegate;
use reactive_graph_plugin_delegates::EntityTypeImportExportManagerDelegate;
use reactive_graph_plugin_delegates::EntityTypeManagerDelegate;
use reactive_graph_plugin_delegates::EntityTypeProviderRegistryDelegate;
use reactive_graph_plugin_delegates::FlowInstanceManagerDelegate;
use reactive_graph_plugin_delegates::FlowTypeImportExportManagerDelegate;
use reactive_graph_plugin_delegates::FlowTypeManagerDelegate;
use reactive_graph_plugin_delegates::FlowTypeProviderRegistryDelegate;
use reactive_graph_plugin_delegates::GraphQLQueryServiceDelegate;
use reactive_graph_plugin_delegates::RelationBehaviourRegistryDelegate;
use reactive_graph_plugin_delegates::RelationComponentBehaviourRegistryDelegate;
use reactive_graph_plugin_delegates::RelationInstanceManagerDelegate;
use reactive_graph_plugin_delegates::RelationTypeImportExportManagerDelegate;
use reactive_graph_plugin_delegates::RelationTypeManagerDelegate;
use reactive_graph_plugin_delegates::RelationTypeProviderRegistryDelegate;
use reactive_graph_plugin_delegates::TypeSystemEventManagerDelegate;
use reactive_graph_plugin_delegates::WebResourceManagerDelegate;
use reactive_graph_plugin_service_api::PluginContextFactory;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_runtime_web_api::WebResourceManager;
use reactive_graph_type_system_api::ComponentImportExportManager;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::ComponentProviderRegistry;
use reactive_graph_type_system_api::EntityTypeImportExportManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::EntityTypeProviderRegistry;
use reactive_graph_type_system_api::FlowTypeImportExportManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::FlowTypeProviderRegistry;
use reactive_graph_type_system_api::RelationTypeImportExportManager;
use reactive_graph_type_system_api::RelationTypeManager;
use reactive_graph_type_system_api::RelationTypeProviderRegistry;
use reactive_graph_type_system_api::TypeSystemEventManager;

use crate::PluginContextImpl;

pub type PluginContextStorage = RwLock<Option<Arc<dyn PluginContext + Send + Sync>>>;

fn create_plugin_context_storage() -> PluginContextStorage {
    RwLock::new(None)
}

#[derive(Component)]
pub struct PluginContextFactoryImpl {
    // Type System
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    component_import_export_manager: Arc<dyn ComponentImportExportManager + Send + Sync>,
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    entity_type_import_export_manager: Arc<dyn EntityTypeImportExportManager + Send + Sync>,
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    relation_type_import_export_manager: Arc<dyn RelationTypeImportExportManager + Send + Sync>,
    relation_type_provider_registry: Arc<dyn RelationTypeProviderRegistry + Send + Sync>,
    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
    flow_type_import_export_manager: Arc<dyn FlowTypeImportExportManager + Send + Sync>,
    flow_type_provider_registry: Arc<dyn FlowTypeProviderRegistry + Send + Sync>,
    type_system_event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,
    // Instance System
    reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,
    reactive_relation_manager: Arc<dyn ReactiveRelationManager + Send + Sync>,
    reactive_flow_manager: Arc<dyn ReactiveFlowManager + Send + Sync>,
    // Behaviour Managers
    entity_behaviour_manager: Arc<dyn EntityBehaviourManager + Send + Sync>,
    entity_component_behaviour_manager: Arc<dyn EntityComponentBehaviourManager + Send + Sync>,
    relation_behaviour_manager: Arc<dyn RelationBehaviourManager + Send + Sync>,
    relation_component_behaviour_manager: Arc<dyn RelationComponentBehaviourManager + Send + Sync>,
    // Behaviour Registries
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,
    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,
    relation_behaviour_registry: Arc<dyn RelationBehaviourRegistry + Send + Sync>,
    relation_component_behaviour_registry: Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>,
    // GraphQL Services
    graphql_query_service: Arc<dyn GraphQLQueryService + Send + Sync>,
    web_resource_manager: Arc<dyn WebResourceManager + Send + Sync>,
    // System Services
    config_manager: Arc<dyn ConfigManager + Send + Sync>,
    command_manager: Arc<dyn CommandManager + Send + Sync>,

    /// The plugin context.
    #[component(default = "create_plugin_context_storage")]
    pub plugin_context: PluginContextStorage,
}

#[async_trait]
#[component_alias]
impl PluginContextFactory for PluginContextFactoryImpl {
    fn construct_plugin_context(&self) {
        // Type System
        let component_manager = ComponentManagerDelegate::new(self.component_manager.clone());
        let component_import_export_manager = ComponentImportExportManagerDelegate::new(self.component_import_export_manager.clone());
        let component_provider_registry = ComponentProviderRegistryDelegate::new(&self.component_provider_registry);
        let entity_type_manager = EntityTypeManagerDelegate::new(self.entity_type_manager.clone());
        let entity_type_import_export_manager = EntityTypeImportExportManagerDelegate::new(self.entity_type_import_export_manager.clone());
        let entity_type_provider_registry = EntityTypeProviderRegistryDelegate::new(&self.entity_type_provider_registry);
        let relation_type_manager = RelationTypeManagerDelegate::new(self.relation_type_manager.clone());
        let relation_type_import_export_manager = RelationTypeImportExportManagerDelegate::new(self.relation_type_import_export_manager.clone());
        let relation_type_provider_registry = RelationTypeProviderRegistryDelegate::new(&self.relation_type_provider_registry);
        let flow_type_manager = FlowTypeManagerDelegate::new(self.flow_type_manager.clone());
        let flow_type_import_export_manager = FlowTypeImportExportManagerDelegate::new(self.flow_type_import_export_manager.clone());
        let flow_type_provider_registry = FlowTypeProviderRegistryDelegate::new(&self.flow_type_provider_registry);
        // Instance System
        let entity_instance_manager =
            EntityInstanceManagerDelegate::new(self.component_manager.clone(), self.entity_type_manager.clone(), self.reactive_entity_manager.clone());
        let relation_instance_manager =
            RelationInstanceManagerDelegate::new(self.component_manager.clone(), self.relation_type_manager.clone(), self.reactive_relation_manager.clone());
        let flow_instance_manager = FlowInstanceManagerDelegate::new(self.reactive_flow_manager.clone());
        // Behaviour Registries
        let entity_behaviour_registry = EntityBehaviourRegistryDelegate::new(
            self.entity_behaviour_manager.clone(),
            self.entity_behaviour_registry.clone(),
            self.reactive_entity_manager.clone(),
        );
        let entity_component_behaviour_registry = EntityComponentBehaviourRegistryDelegate::new(
            self.entity_component_behaviour_manager.clone(),
            self.entity_component_behaviour_registry.clone(),
            self.reactive_entity_manager.clone(),
        );
        let relation_behaviour_registry = RelationBehaviourRegistryDelegate::new(
            self.relation_behaviour_manager.clone(),
            self.relation_behaviour_registry.clone(),
            self.reactive_relation_manager.clone(),
        );
        let relation_component_behaviour_registry = RelationComponentBehaviourRegistryDelegate::new(
            self.relation_component_behaviour_manager.clone(),
            self.relation_component_behaviour_registry.clone(),
            self.reactive_relation_manager.clone(),
        );
        // GraphQL Services
        let graphql_query_service = GraphQLQueryServiceDelegate::new(self.graphql_query_service.clone());
        let web_resource_manager = WebResourceManagerDelegate::new(self.web_resource_manager.clone());
        // System Services
        let config_manager = ConfigManagerDelegate::new(self.config_manager.clone());
        let type_system_event_manager = TypeSystemEventManagerDelegate::new(self.type_system_event_manager.clone());
        let command_manager = CommandManagerDelegate::new(self.command_manager.clone());
        let plugin_context = PluginContextImpl::new(
            Arc::new(component_manager),
            Arc::new(component_import_export_manager),
            Arc::new(component_provider_registry),
            Arc::new(entity_type_manager),
            Arc::new(entity_type_import_export_manager),
            Arc::new(entity_type_provider_registry),
            Arc::new(relation_type_manager),
            Arc::new(relation_type_import_export_manager),
            Arc::new(relation_type_provider_registry),
            Arc::new(flow_type_manager),
            Arc::new(flow_type_import_export_manager),
            Arc::new(flow_type_provider_registry),
            Arc::new(type_system_event_manager),
            Arc::new(entity_instance_manager),
            Arc::new(relation_instance_manager),
            Arc::new(flow_instance_manager),
            Arc::new(entity_behaviour_registry),
            Arc::new(entity_component_behaviour_registry),
            Arc::new(relation_behaviour_registry),
            Arc::new(relation_component_behaviour_registry),
            Arc::new(config_manager),
            Arc::new(graphql_query_service),
            Arc::new(web_resource_manager),
            Arc::new(command_manager),
        );
        let plugin_context = Arc::new(plugin_context);
        let mut writer = self.plugin_context.write().unwrap();
        let _ = writer.insert(plugin_context);
    }

    fn get(&self) -> Option<Arc<dyn PluginContext + Send + Sync>> {
        let reader = self.plugin_context.read().unwrap();
        if let Some(plugin_context) = reader.as_ref() {
            return Some(plugin_context.clone());
        }
        None
    }
}

#[async_trait]
impl Lifecycle for PluginContextFactoryImpl {
    async fn init(&self) {
        self.construct_plugin_context();
    }

    async fn shutdown(&self) {}
}
