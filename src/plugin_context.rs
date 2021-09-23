use crate::entity_instance_creator::EntityInstanceCreator;
use crate::relation_instance_creator::RelationInstanceCreator;
use std::sync::Arc;

pub trait PluginContext: Send + Sync {
    fn get_entity_instance_creator(&self) -> Arc<dyn EntityInstanceCreator>;

    fn get_relation_instance_creator(&self) -> Arc<dyn RelationInstanceCreator>;
}
