use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_behaviour_service_api::BehaviourSystem;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::TypeSystemSystem;

use crate::ReactiveEntityManager;
use crate::ReactiveFlowManager;
use crate::ReactiveInstanceEventManager;
use crate::ReactiveRelationManager;

#[injectable]
#[async_trait]
pub trait ReactiveSystem: Lifecycle {
    fn get_reactive_entity_manager(&self) -> Arc<dyn ReactiveEntityManager + Send + Sync>;

    fn get_reactive_flow_manager(&self) -> Arc<dyn ReactiveFlowManager + Send + Sync>;

    fn get_reactive_relation_manager(&self) -> Arc<dyn ReactiveRelationManager + Send + Sync>;

    fn get_reactive_instance_event_manager(&self) -> Arc<dyn ReactiveInstanceEventManager + Send + Sync>;

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync>;

    fn behaviour_system(&self) -> Arc<dyn BehaviourSystem + Send + Sync>;
}
