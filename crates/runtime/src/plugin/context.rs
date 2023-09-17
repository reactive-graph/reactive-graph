use std::sync::Arc;

use crate::plugins::CommandManager;
use crate::plugins::ComponentImportExportManager;
use crate::plugins::ComponentManager;
use crate::plugins::ComponentProviderRegistry;
use crate::plugins::ConfigManager;
use crate::plugins::EntityBehaviourRegistry;
use crate::plugins::EntityComponentBehaviourRegistry;
use crate::plugins::EntityInstanceManager;
use crate::plugins::EntityTypeImportExportManager;
use crate::plugins::EntityTypeManager;
use crate::plugins::EntityTypeProviderRegistry;
use crate::plugins::FlowInstanceManager;
use crate::plugins::FlowTypeImportExportManager;
use crate::plugins::FlowTypeManager;
use crate::plugins::FlowTypeProviderRegistry;
use crate::plugins::GraphQLQueryService;
use crate::plugins::PluginContext;
use crate::plugins::RelationBehaviourRegistry;
use crate::plugins::RelationComponentBehaviourRegistry;
use crate::plugins::RelationInstanceManager;
use crate::plugins::RelationTypeImportExportManager;
use crate::plugins::RelationTypeManager;
use crate::plugins::RelationTypeProviderRegistry;
use crate::plugins::SystemEventManager;
use crate::plugins::WebResourceManager;

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
    system_event_manager: Arc<dyn SystemEventManager + Send + Sync>,
    command_manager: Arc<dyn CommandManager + Send + Sync>,
}

impl PluginContextImpl {
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
        system_event_manager: Arc<dyn SystemEventManager + Send + Sync>,
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
            system_event_manager,
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

    fn get_config_manager(&self) -> Arc<dyn ConfigManager + Send + Sync> {
        self.config_manager.clone()
    }

    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService + Send + Sync> {
        self.graphql_query_service.clone()
    }

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager + Send + Sync> {
        self.web_resource_manager.clone()
    }

    fn get_system_event_manager(&self) -> Arc<dyn SystemEventManager + Send + Sync> {
        self.system_event_manager.clone()
    }

    fn get_command_manager(&self) -> Arc<dyn CommandManager + Send + Sync> {
        self.command_manager.clone()
    }
}
