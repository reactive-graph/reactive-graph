use std::sync::Arc;

use crate::ComponentManager;
use crate::EntityInstanceManager;
use crate::EntityTypeManager;
use crate::FlowInstanceManager;
use crate::RelationInstanceManager;
use crate::RelationTypeManager;

pub trait PluginContext: Send + Sync {
    /// Returns the component manager.
    fn get_component_manager(&self) -> Arc<dyn ComponentManager>;

    /// Returns the entity type manager.
    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager>;

    /// Returns the relation type manager.
    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager>;

    /// Returns the entity instance manager.
    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager>;

    /// Returns the relation instance manager.
    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager>;

    /// Returns the flow instance manager.
    fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager>;
}
