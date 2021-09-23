use crate::entity_instance_creator::EntityInstanceCreator;
use crate::relation_instance_creator::RelationInstanceCreator;
use std::sync::Arc;

pub trait PluginContext: Send + Sync {
    fn init(
        &self,
        entity_instance_creator: Arc<dyn EntityInstanceCreator>,
        relation_instance_creator: Arc<dyn RelationInstanceCreator>,
    );

    fn get_entity_instance_creator(&self) -> Arc<dyn EntityInstanceCreator>;

    fn get_relation_instance_creator(&self) -> Arc<dyn RelationInstanceCreator>;
}
