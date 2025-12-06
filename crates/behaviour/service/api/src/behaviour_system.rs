use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::TypeSystemSystem;

use crate::EntityBehaviourManager;
use crate::EntityBehaviourRegistry;
use crate::EntityComponentBehaviourManager;
use crate::EntityComponentBehaviourRegistry;
use crate::RelationBehaviourManager;
use crate::RelationBehaviourRegistry;
use crate::RelationComponentBehaviourManager;
use crate::RelationComponentBehaviourRegistry;

#[injectable]
#[async_trait]
pub trait BehaviourSystem: Lifecycle {
    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager + Send + Sync>;

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry + Send + Sync>;

    fn get_entity_component_behaviour_manager(&self) -> Arc<dyn EntityComponentBehaviourManager + Send + Sync>;

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>;

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager + Send + Sync>;

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry + Send + Sync>;

    fn get_relation_component_behaviour_manager(&self) -> Arc<dyn RelationComponentBehaviourManager + Send + Sync>;

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>;

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync>;
}
