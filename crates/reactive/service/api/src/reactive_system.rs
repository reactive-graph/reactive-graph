use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_behaviour_service_api::BehaviourSystem;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_type_system_api::TypeSystem;

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

    fn type_system(&self) -> Arc<dyn TypeSystem + Send + Sync>;

    fn behaviour_system(&self) -> Arc<dyn BehaviourSystem + Send + Sync>;
}
