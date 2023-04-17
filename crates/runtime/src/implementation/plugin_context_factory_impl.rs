use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::api::CommandManager;
use crate::api::ComponentManager;
use crate::api::ConfigManager;
use crate::api::EntityBehaviourManager;
use crate::api::EntityBehaviourRegistry;
use crate::api::EntityComponentBehaviourManager;
use crate::api::EntityComponentBehaviourRegistry;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::GraphQLQueryService;
use crate::api::Lifecycle;
use crate::api::PluginContextFactory;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveFlowInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationBehaviourManager;
use crate::api::RelationBehaviourRegistry;
use crate::api::RelationComponentBehaviourManager;
use crate::api::RelationComponentBehaviourRegistry;
use crate::api::RelationTypeManager;
use crate::api::SystemEventManager;
use crate::di::*;
use crate::plugin::CommandManagerImpl;
use crate::plugin::ComponentManagerImpl;
use crate::plugin::ConfigManagerImpl;
use crate::plugin::EntityBehaviourRegistryImpl;
use crate::plugin::EntityComponentBehaviourRegistryImpl;
use crate::plugin::EntityInstanceManagerImpl;
use crate::plugin::EntityTypeManagerImpl;
use crate::plugin::FlowInstanceManagerImpl;
use crate::plugin::FlowTypeManagerImpl;
use crate::plugin::GraphQLQueryServiceImpl;
use crate::plugin::PluginContextImpl;
use crate::plugin::RelationBehaviourRegistryImpl;
use crate::plugin::RelationComponentBehaviourRegistryImpl;
use crate::plugin::RelationInstanceManagerImpl;
use crate::plugin::RelationTypeManagerImpl;
use crate::plugin::SystemEventManagerImpl;
use crate::plugins::PluginContext;

#[wrapper]
pub struct PluginContextStorage(RwLock<Option<Arc<dyn PluginContext>>>);

#[provides]
fn create_plugin_context_storage() -> PluginContextStorage {
    PluginContextStorage(RwLock::new(None))
}

#[component]
pub struct PluginContextFactoryImpl {
    // Type System
    component_manager: Wrc<dyn ComponentManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    flow_type_manager: Wrc<dyn FlowTypeManager>,
    // Instance System
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,
    reactive_relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,
    reactive_flow_instance_manager: Wrc<dyn ReactiveFlowInstanceManager>,
    // Behaviour Managers
    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,
    entity_component_behaviour_manager: Wrc<dyn EntityComponentBehaviourManager>,
    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,
    relation_component_behaviour_manager: Wrc<dyn RelationComponentBehaviourManager>,
    // Behaviour Registries
    entity_behaviour_registry: Wrc<dyn EntityBehaviourRegistry>,
    entity_component_behaviour_registry: Wrc<dyn EntityComponentBehaviourRegistry>,
    relation_behaviour_registry: Wrc<dyn RelationBehaviourRegistry>,
    relation_component_behaviour_registry: Wrc<dyn RelationComponentBehaviourRegistry>,
    // System Services
    config_manager: Wrc<dyn ConfigManager>,
    graphql_query_service: Wrc<dyn GraphQLQueryService>,
    system_event_manager: Wrc<dyn SystemEventManager>,
    command_manager: Wrc<dyn CommandManager>,

    /// The plugin context.
    pub plugin_context: PluginContextStorage,
}

impl PluginContextFactoryImpl {}

#[async_trait]
#[provides]
impl PluginContextFactory for PluginContextFactoryImpl {
    fn construct_plugin_context(&self) {
        // Type System
        let component_manager = ComponentManagerImpl::new(self.component_manager.clone());
        let entity_type_manager = EntityTypeManagerImpl::new(self.entity_type_manager.clone());
        let relation_type_manager = RelationTypeManagerImpl::new(self.relation_type_manager.clone());
        let flow_type_manager = FlowTypeManagerImpl::new(self.flow_type_manager.clone());
        // Instance System
        let entity_instance_manager = EntityInstanceManagerImpl::new(
            self.component_manager.clone(),
            self.entity_type_manager.clone(),
            self.reactive_entity_instance_manager.clone(),
        );
        let relation_instance_manager = RelationInstanceManagerImpl::new(
            self.component_manager.clone(),
            self.relation_type_manager.clone(),
            self.reactive_relation_instance_manager.clone(),
        );
        let flow_instance_manager = FlowInstanceManagerImpl::new(self.reactive_flow_instance_manager.clone());
        // Behaviour Registries
        let entity_behaviour_registry = EntityBehaviourRegistryImpl::new(
            self.entity_behaviour_manager.clone(),
            self.entity_behaviour_registry.clone(),
            self.reactive_entity_instance_manager.clone(),
        );
        let entity_component_behaviour_registry = EntityComponentBehaviourRegistryImpl::new(
            self.entity_component_behaviour_manager.clone(),
            self.entity_component_behaviour_registry.clone(),
            self.reactive_entity_instance_manager.clone(),
        );
        let relation_behaviour_registry = RelationBehaviourRegistryImpl::new(
            self.relation_behaviour_manager.clone(),
            self.relation_behaviour_registry.clone(),
            self.reactive_relation_instance_manager.clone(),
        );
        let relation_component_behaviour_registry = RelationComponentBehaviourRegistryImpl::new(
            self.relation_component_behaviour_manager.clone(),
            self.relation_component_behaviour_registry.clone(),
            self.reactive_relation_instance_manager.clone(),
        );
        // System Services
        let config_manager = ConfigManagerImpl::new(self.config_manager.clone());
        let graphql_query_service = GraphQLQueryServiceImpl::new(self.graphql_query_service.clone());
        let system_event_manager = SystemEventManagerImpl::new(self.system_event_manager.clone());
        let command_manager = CommandManagerImpl::new(self.command_manager.clone());
        let plugin_context = PluginContextImpl::new(
            Arc::new(component_manager),
            Arc::new(entity_type_manager),
            Arc::new(relation_type_manager),
            Arc::new(flow_type_manager),
            Arc::new(entity_instance_manager),
            Arc::new(relation_instance_manager),
            Arc::new(flow_instance_manager),
            Arc::new(entity_behaviour_registry),
            Arc::new(entity_component_behaviour_registry),
            Arc::new(relation_behaviour_registry),
            Arc::new(relation_component_behaviour_registry),
            Arc::new(config_manager),
            Arc::new(graphql_query_service),
            Arc::new(system_event_manager),
            Arc::new(command_manager),
        );
        let plugin_context = Arc::new(plugin_context);
        let mut writer = self.plugin_context.0.write().unwrap();
        let _ = writer.insert(plugin_context);
    }

    fn get(&self) -> Option<Arc<dyn PluginContext>> {
        let reader = self.plugin_context.0.read().unwrap();
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
