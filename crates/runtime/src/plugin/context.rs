use std::sync::Arc;

use crate::plugins::CommandManager;
use crate::plugins::ComponentManager;
use crate::plugins::ConfigManager;
use crate::plugins::EntityBehaviourRegistry;
use crate::plugins::EntityComponentBehaviourRegistry;
use crate::plugins::EntityInstanceManager;
use crate::plugins::EntityTypeManager;
use crate::plugins::FlowInstanceManager;
use crate::plugins::FlowTypeManager;
use crate::plugins::GraphQLQueryService;
use crate::plugins::PluginContext;
use crate::plugins::RelationBehaviourRegistry;
use crate::plugins::RelationComponentBehaviourRegistry;
use crate::plugins::RelationInstanceManager;
use crate::plugins::RelationTypeManager;
use crate::plugins::SystemEventManager;

pub struct PluginContextImpl {
    component_manager: Arc<dyn ComponentManager>,
    entity_type_manager: Arc<dyn EntityTypeManager>,
    relation_type_manager: Arc<dyn RelationTypeManager>,
    flow_type_manager: Arc<dyn FlowTypeManager>,
    entity_instance_manager: Arc<dyn EntityInstanceManager>,
    relation_instance_manager: Arc<dyn RelationInstanceManager>,
    flow_instance_manager: Arc<dyn FlowInstanceManager>,
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry>,
    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry>,
    relation_behaviour_registry: Arc<dyn RelationBehaviourRegistry>,
    relation_component_behaviour_registry: Arc<dyn RelationComponentBehaviourRegistry>,
    config_manager: Arc<dyn ConfigManager>,
    graphql_query_service: Arc<dyn GraphQLQueryService>,
    system_event_manager: Arc<dyn SystemEventManager>,
    command_manager: Arc<dyn CommandManager>,
}

impl PluginContextImpl {
    pub fn new(
        component_manager: Arc<dyn ComponentManager>,
        entity_type_manager: Arc<dyn EntityTypeManager>,
        relation_type_manager: Arc<dyn RelationTypeManager>,
        flow_type_manager: Arc<dyn FlowTypeManager>,
        entity_instance_manager: Arc<dyn EntityInstanceManager>,
        relation_instance_manager: Arc<dyn RelationInstanceManager>,
        flow_instance_manager: Arc<dyn FlowInstanceManager>,
        entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry>,
        entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry>,
        relation_behaviour_registry: Arc<dyn RelationBehaviourRegistry>,
        relation_component_behaviour_registry: Arc<dyn RelationComponentBehaviourRegistry>,
        config_manager: Arc<dyn ConfigManager>,
        graphql_query_service: Arc<dyn GraphQLQueryService>,
        system_event_manager: Arc<dyn SystemEventManager>,
        command_manager: Arc<dyn CommandManager>,
    ) -> Self {
        PluginContextImpl {
            component_manager,
            entity_type_manager,
            relation_type_manager,
            flow_type_manager,
            entity_instance_manager,
            relation_instance_manager,
            flow_instance_manager,
            entity_behaviour_registry,
            entity_component_behaviour_registry,
            relation_behaviour_registry,
            relation_component_behaviour_registry,
            config_manager,
            graphql_query_service,
            system_event_manager,
            command_manager,
        }
    }
}

impl PluginContext for PluginContextImpl {
    fn get_component_manager(&self) -> Arc<dyn ComponentManager> {
        self.component_manager.clone()
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager> {
        self.entity_type_manager.clone()
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager> {
        self.relation_type_manager.clone()
    }

    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager> {
        self.flow_type_manager.clone()
    }

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager> {
        self.entity_instance_manager.clone()
    }

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager> {
        self.relation_instance_manager.clone()
    }

    fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager> {
        self.flow_instance_manager.clone()
    }

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry> {
        self.entity_behaviour_registry.clone()
    }

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry> {
        self.entity_component_behaviour_registry.clone()
    }

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry> {
        self.relation_behaviour_registry.clone()
    }

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry> {
        self.relation_component_behaviour_registry.clone()
    }

    fn get_config_manager(&self) -> Arc<dyn ConfigManager> {
        self.config_manager.clone()
    }

    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService> {
        self.graphql_query_service.clone()
    }

    fn get_system_event_manager(&self) -> Arc<dyn SystemEventManager> {
        self.system_event_manager.clone()
    }

    fn get_command_manager(&self) -> Arc<dyn CommandManager> {
        self.command_manager.clone()
    }
}
