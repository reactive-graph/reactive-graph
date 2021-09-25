use crate::{
    ComponentManager, EntityInstanceManager, EntityTypeManager, FlowManager,
    RelationInstanceManager, RelationTypeManager,
};
use std::sync::Arc;

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

    /// Returns the flow manager.
    fn get_flow_manager(&self) -> Arc<dyn FlowManager>;
}
