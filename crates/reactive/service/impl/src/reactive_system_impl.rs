use std::sync::Arc;

use async_trait::async_trait;
use reactive_graph_behaviour_service_api::BehaviourSystem;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_reactive_service_api::ReactiveInstanceEventManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_reactive_service_api::ReactiveSystem;
use reactive_graph_type_system_api::TypeSystem;

#[derive(Component)]
pub struct ReactiveSystemImpl {
    reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,
    reactive_relation_manager: Arc<dyn ReactiveRelationManager + Send + Sync>,
    reactive_flow_manager: Arc<dyn ReactiveFlowManager + Send + Sync>,
    reactive_instance_event_manager: Arc<dyn ReactiveInstanceEventManager + Send + Sync>,

    type_system: Arc<dyn TypeSystem + Send + Sync>,
    behaviour_system: Arc<dyn BehaviourSystem + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl ReactiveSystem for ReactiveSystemImpl {
    fn get_reactive_entity_manager(&self) -> Arc<dyn ReactiveEntityManager + Send + Sync> {
        self.reactive_entity_manager.clone()
    }

    fn get_reactive_flow_manager(&self) -> Arc<dyn ReactiveFlowManager + Send + Sync> {
        self.reactive_flow_manager.clone()
    }

    fn get_reactive_relation_manager(&self) -> Arc<dyn ReactiveRelationManager + Send + Sync> {
        self.reactive_relation_manager.clone()
    }

    fn get_reactive_instance_event_manager(&self) -> Arc<dyn ReactiveInstanceEventManager + Send + Sync> {
        self.reactive_instance_event_manager.clone()
    }

    fn type_system(&self) -> Arc<dyn TypeSystem + Send + Sync> {
        self.type_system.clone()
    }

    fn behaviour_system(&self) -> Arc<dyn BehaviourSystem + Send + Sync> {
        self.behaviour_system.clone()
    }
}

#[async_trait]
impl Lifecycle for ReactiveSystemImpl {
    async fn init(&self) {
        self.reactive_entity_manager.init().await;
        self.reactive_relation_manager.init().await;
        self.reactive_flow_manager.init().await;
        self.reactive_instance_event_manager.init().await;
    }

    async fn post_init(&self) {
        self.reactive_entity_manager.post_init().await;
        self.reactive_relation_manager.post_init().await;
        self.reactive_flow_manager.post_init().await;
        self.reactive_instance_event_manager.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.reactive_instance_event_manager.pre_shutdown().await;
        self.reactive_flow_manager.pre_shutdown().await;
        self.reactive_relation_manager.pre_shutdown().await;
        self.reactive_entity_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.reactive_instance_event_manager.shutdown().await;
        self.reactive_flow_manager.shutdown().await;
        self.reactive_relation_manager.shutdown().await;
        self.reactive_entity_manager.shutdown().await;
    }
}
