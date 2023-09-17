use std::sync::Arc;

use crate::springtime_di::injectable;
use crate::CommandManager;
use crate::ComponentImportExportManager;
use crate::ComponentManager;
use crate::ComponentProviderRegistry;
use crate::ConfigManager;
use crate::EntityBehaviourRegistry;
use crate::EntityComponentBehaviourRegistry;
use crate::EntityInstanceManager;
use crate::EntityTypeImportExportManager;
use crate::EntityTypeManager;
use crate::EntityTypeProviderRegistry;
use crate::FlowInstanceManager;
use crate::FlowTypeImportExportManager;
use crate::FlowTypeManager;
use crate::FlowTypeProviderRegistry;
use crate::GraphQLQueryService;
use crate::RelationBehaviourRegistry;
use crate::RelationComponentBehaviourRegistry;
use crate::RelationInstanceManager;
use crate::RelationTypeImportExportManager;
use crate::RelationTypeManager;
use crate::RelationTypeProviderRegistry;
use crate::SystemEventManager;
use crate::WebResourceManager;

// #[injectable]
#[cfg_attr(feature = "springtime", injectable)]
pub trait PluginContext: Send + Sync {
    // Type System

    /// Returns the component manager.
    fn get_component_manager(&self) -> Arc<dyn ComponentManager + Send + Sync>;

    /// Returns the component import export manager.
    fn get_component_import_export_manager(&self) -> Arc<dyn ComponentImportExportManager + Send + Sync>;

    /// Returns the component provider registry.
    fn get_component_provider_registry(&self) -> Arc<dyn ComponentProviderRegistry + Send + Sync>;

    /// Returns the entity type manager.
    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager + Send + Sync>;

    /// Returns the entity type import export manager.
    fn get_entity_type_import_export_manager(&self) -> Arc<dyn EntityTypeImportExportManager + Send + Sync>;

    /// Returns the entity type provider registry.
    fn get_entity_type_provider_registry(&self) -> Arc<dyn EntityTypeProviderRegistry + Send + Sync>;

    /// Returns the relation type manager.
    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager + Send + Sync>;

    /// Returns the relation type import export manager.
    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager + Send + Sync>;

    /// Returns the relation type provider registry.
    fn get_relation_type_provider_registry(&self) -> Arc<dyn RelationTypeProviderRegistry + Send + Sync>;

    /// Returns the flow type manager.
    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager + Send + Sync>;

    /// Returns the flow type import export manager.
    fn get_flow_type_import_export_manager(&self) -> Arc<dyn FlowTypeImportExportManager + Send + Sync>;

    /// Returns the flow type provider registry.
    fn get_flow_type_provider_registry(&self) -> Arc<dyn FlowTypeProviderRegistry + Send + Sync>;

    // Instance System

    /// Returns the entity instance manager.
    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager + Send + Sync>;

    /// Returns the relation instance manager.
    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager + Send + Sync>;

    /// Returns the flow instance manager.
    fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager + Send + Sync>;

    // Behaviour Registries

    /// Returns the entity behaviour registry.
    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry + Send + Sync>;

    /// Returns the entity component behaviour registry.
    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>;

    /// Returns the relation behaviour registry.
    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry + Send + Sync>;

    /// Returns the relation component behaviour registry.
    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>;

    // GraphQL Services

    /// Returns the GraphQL query service.
    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService + Send + Sync>;

    /// Returns the web resource manager.
    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager + Send + Sync>;

    // System Services

    /// Returns the config manager.
    fn get_config_manager(&self) -> Arc<dyn ConfigManager + Send + Sync>;

    /// Returns the system event manager.
    fn get_system_event_manager(&self) -> Arc<dyn SystemEventManager + Send + Sync>;

    /// Returns the command manager.
    fn get_command_manager(&self) -> Arc<dyn CommandManager + Send + Sync>;
}
