use std::sync::Arc;

use reactive_graph_plugin_api::CommandManager;
use reactive_graph_plugin_api::ComponentImportExportManager;
use reactive_graph_plugin_api::ComponentManager;
use reactive_graph_plugin_api::ComponentProviderRegistry;
use reactive_graph_plugin_api::ConfigManager;
use reactive_graph_plugin_api::EntityBehaviourRegistry;
use reactive_graph_plugin_api::EntityComponentBehaviourRegistry;
use reactive_graph_plugin_api::EntityInstanceManager;
use reactive_graph_plugin_api::EntityTypeImportExportManager;
use reactive_graph_plugin_api::EntityTypeManager;
use reactive_graph_plugin_api::EntityTypeProviderRegistry;
use reactive_graph_plugin_api::FlowInstanceManager;
use reactive_graph_plugin_api::FlowTypeImportExportManager;
use reactive_graph_plugin_api::FlowTypeManager;
use reactive_graph_plugin_api::FlowTypeProviderRegistry;
use reactive_graph_plugin_api::GraphQLQueryService;
use reactive_graph_plugin_api::PluginContext;
use reactive_graph_plugin_api::RelationBehaviourRegistry;
use reactive_graph_plugin_api::RelationComponentBehaviourRegistry;
use reactive_graph_plugin_api::RelationInstanceManager;
use reactive_graph_plugin_api::RelationTypeImportExportManager;
use reactive_graph_plugin_api::RelationTypeManager;
use reactive_graph_plugin_api::RelationTypeProviderRegistry;
use reactive_graph_plugin_api::TypeSystemEventManager;
use reactive_graph_plugin_api::WebResourceManager;

#[derive(Clone)]
pub struct PluginContextImpl {
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
    entity_instance_manager: Arc<dyn EntityInstanceManager + Send + Sync>,
    relation_instance_manager: Arc<dyn RelationInstanceManager + Send + Sync>,
    flow_instance_manager: Arc<dyn FlowInstanceManager + Send + Sync>,
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,
    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,
    relation_behaviour_registry: Arc<dyn RelationBehaviourRegistry + Send + Sync>,
    relation_component_behaviour_registry: Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>,
    config_manager: Arc<dyn ConfigManager + Send + Sync>,
    graphql_query_service: Arc<dyn GraphQLQueryService + Send + Sync>,
    web_resource_manager: Arc<dyn WebResourceManager + Send + Sync>,
    command_manager: Arc<dyn CommandManager + Send + Sync>,
}

impl PluginContextImpl {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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
        entity_instance_manager: Arc<dyn EntityInstanceManager + Send + Sync>,
        relation_instance_manager: Arc<dyn RelationInstanceManager + Send + Sync>,
        flow_instance_manager: Arc<dyn FlowInstanceManager + Send + Sync>,
        entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,
        entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,
        relation_behaviour_registry: Arc<dyn RelationBehaviourRegistry + Send + Sync>,
        relation_component_behaviour_registry: Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>,
        config_manager: Arc<dyn ConfigManager + Send + Sync>,
        graphql_query_service: Arc<dyn GraphQLQueryService + Send + Sync>,
        web_resource_manager: Arc<dyn WebResourceManager + Send + Sync>,
        command_manager: Arc<dyn CommandManager + Send + Sync>,
    ) -> Self {
        PluginContextImpl {
            component_manager,
            component_import_export_manager,
            component_provider_registry,
            entity_type_manager,
            entity_type_import_export_manager,
            entity_type_provider_registry,
            relation_type_manager,
            relation_type_import_export_manager,
            relation_type_provider_registry,
            flow_type_manager,
            flow_type_import_export_manager,
            flow_type_provider_registry,
            type_system_event_manager,
            entity_instance_manager,
            relation_instance_manager,
            flow_instance_manager,
            entity_behaviour_registry,
            entity_component_behaviour_registry,
            relation_behaviour_registry,
            relation_component_behaviour_registry,
            config_manager,
            graphql_query_service,
            web_resource_manager,
            command_manager,
        }
    }
}

impl PluginContext for PluginContextImpl {
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

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager + Send + Sync> {
        self.relation_type_manager.clone()
    }

    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager + Send + Sync> {
        self.relation_type_import_export_manager.clone()
    }

    fn get_relation_type_provider_registry(&self) -> Arc<dyn RelationTypeProviderRegistry + Send + Sync> {
        self.relation_type_provider_registry.clone()
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

    fn get_type_system_event_manager(&self) -> Arc<dyn TypeSystemEventManager + Send + Sync> {
        self.type_system_event_manager.clone()
    }

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager + Send + Sync> {
        self.entity_instance_manager.clone()
    }

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager + Send + Sync> {
        self.relation_instance_manager.clone()
    }

    fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager + Send + Sync> {
        self.flow_instance_manager.clone()
    }

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry + Send + Sync> {
        self.entity_behaviour_registry.clone()
    }

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry + Send + Sync> {
        self.entity_component_behaviour_registry.clone()
    }

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry + Send + Sync> {
        self.relation_behaviour_registry.clone()
    }

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry + Send + Sync> {
        self.relation_component_behaviour_registry.clone()
    }

    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService + Send + Sync> {
        self.graphql_query_service.clone()
    }

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager + Send + Sync> {
        self.web_resource_manager.clone()
    }

    fn get_config_manager(&self) -> Arc<dyn ConfigManager + Send + Sync> {
        self.config_manager.clone()
    }

    fn get_command_manager(&self) -> Arc<dyn CommandManager + Send + Sync> {
        self.command_manager.clone()
    }
}
