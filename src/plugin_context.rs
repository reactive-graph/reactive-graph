use crate::EntityInstanceManager;
use crate::FlowManager;
use crate::RelationInstanceManager;
use std::sync::Arc;

pub trait PluginContext: Send + Sync {
    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager>;

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager>;

    fn get_flow_manager(&self) -> Arc<dyn FlowManager>;
}
