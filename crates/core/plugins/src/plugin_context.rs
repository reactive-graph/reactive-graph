use std::sync::Arc;

use crate::{CommandManager, EntityTypeImportExportManager, FlowTypeImportExportManager, RelationTypeImportExportManager};
use crate::ComponentManager;
use crate::ComponentImportExportManager;
use crate::ConfigManager;
use crate::EntityBehaviourRegistry;
use crate::EntityComponentBehaviourRegistry;
use crate::EntityInstanceManager;
use crate::EntityTypeManager;
use crate::FlowInstanceManager;
use crate::FlowTypeManager;
use crate::GraphQLQueryService;
use crate::RelationBehaviourRegistry;
use crate::RelationComponentBehaviourRegistry;
use crate::RelationInstanceManager;
use crate::RelationTypeManager;
use crate::SystemEventManager;

#[derive(Debug)]
pub enum PluginContextInitializationError {
    InitializationError,
}

#[derive(Debug)]
pub enum PluginContextDeinitializationError {
    DeinitializationError,
}

pub trait PluginContext: Send + Sync {
    // Type System

    /// Returns the component manager.
    fn get_component_manager(&self) -> Arc<dyn ComponentManager>;

    /// Returns the component import export manager.
    fn get_component_import_export_manager(&self) -> Arc<dyn ComponentImportExportManager>;

    /// Returns the entity type manager.
    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager>;

    /// Returns the entity type import export manager.
    fn get_entity_type_import_export_manager(&self) -> Arc<dyn EntityTypeImportExportManager>;

    /// Returns the relation type manager.
    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager>;

    /// Returns the relation type import export manager.
    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager>;

    /// Returns the flow type manager.
    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager>;

    /// Returns the flow type import export manager.
    fn get_flow_type_import_export_manager(&self) -> Arc<dyn FlowTypeImportExportManager>;

    // Instance System

    /// Returns the entity instance manager.
    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager>;

    /// Returns the relation instance manager.
    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager>;

    /// Returns the flow instance manager.
    fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager>;

    // Behaviour Registries

    /// Returns the entity behaviour registry.
    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry>;

    /// Returns the entity component behaviour registry.
    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry>;

    /// Returns the relation behaviour registry.
    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry>;

    /// Returns the relation component behaviour registry.
    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry>;

    // System Services

    /// Returns the config manager.
    fn get_config_manager(&self) -> Arc<dyn ConfigManager>;

    /// Returns the GraphQL query service.
    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService>;

    /// Returns the system event manager.
    fn get_system_event_manager(&self) -> Arc<dyn SystemEventManager>;

    /// Returns the command manager.
    fn get_command_manager(&self) -> Arc<dyn CommandManager>;
}
