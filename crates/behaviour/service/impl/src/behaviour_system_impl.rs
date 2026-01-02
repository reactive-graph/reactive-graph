use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_behaviour_service_api::BehaviourSystem;
use reactive_graph_behaviour_service_api::EntityBehaviourManager;
use reactive_graph_behaviour_service_api::EntityBehaviourRegistry;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourManager;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourRegistry;
use reactive_graph_behaviour_service_api::RelationBehaviourManager;
use reactive_graph_behaviour_service_api::RelationBehaviourRegistry;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourManager;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourRegistry;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::TypeSystemSystem;

#[derive(Component)]
pub struct BehaviourSystemImpl {
    entity_behaviour_manager: Arc<dyn EntityBehaviourManager + Send + Sync>,
    entity_behaviour_registry: Arc<dyn EntityBehaviourRegistry + Send + Sync>,
    entity_component_behaviour_manager: Arc<dyn EntityComponentBehaviourManager + Send + Sync>,
    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,
    relation_behaviour_manager: Arc<dyn RelationBehaviourManager + Send + Sync>,
    relation_behaviour_registry: Arc<dyn RelationBehaviourRegistry + Send + Sync>,
    relation_component_behaviour_manager: Arc<dyn RelationComponentBehaviourManager + Send + Sync>,
    relation_component_behaviour_registry: Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>,

    type_system_system: Arc<dyn TypeSystemSystem + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl BehaviourSystem for BehaviourSystemImpl {
    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager + Send + Sync> {
        self.entity_behaviour_manager.clone()
    }

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry + Send + Sync> {
        self.entity_behaviour_registry.clone()
    }

    fn get_entity_component_behaviour_manager(&self) -> Arc<dyn EntityComponentBehaviourManager + Send + Sync> {
        self.entity_component_behaviour_manager.clone()
    }

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry + Send + Sync> {
        self.entity_component_behaviour_registry.clone()
    }

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager + Send + Sync> {
        self.relation_behaviour_manager.clone()
    }

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry + Send + Sync> {
        self.relation_behaviour_registry.clone()
    }

    fn get_relation_component_behaviour_manager(&self) -> Arc<dyn RelationComponentBehaviourManager + Send + Sync> {
        self.relation_component_behaviour_manager.clone()
    }

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry + Send + Sync> {
        self.relation_component_behaviour_registry.clone()
    }

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync> {
        self.type_system_system.clone()
    }
}

#[async_trait]
impl Lifecycle for BehaviourSystemImpl {
    async fn init(&self) {
        self.entity_behaviour_registry.init().await;
        self.entity_component_behaviour_registry.init().await;
        self.relation_behaviour_registry.init().await;
        self.relation_component_behaviour_registry.init().await;
        self.entity_behaviour_manager.init().await;
        self.entity_component_behaviour_manager.init().await;
        self.relation_behaviour_manager.init().await;
        self.relation_component_behaviour_manager.init().await;
    }

    async fn post_init(&self) {
        self.entity_behaviour_registry.post_init().await;
        self.entity_component_behaviour_registry.post_init().await;
        self.relation_behaviour_registry.post_init().await;
        self.relation_component_behaviour_registry.post_init().await;
        self.entity_behaviour_manager.post_init().await;
        self.entity_component_behaviour_manager.post_init().await;
        self.relation_behaviour_manager.post_init().await;
        self.relation_component_behaviour_manager.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.relation_component_behaviour_manager.pre_shutdown().await;
        self.relation_behaviour_manager.pre_shutdown().await;
        self.entity_component_behaviour_manager.pre_shutdown().await;
        self.entity_behaviour_manager.pre_shutdown().await;
        self.relation_component_behaviour_registry.pre_shutdown().await;
        self.relation_behaviour_registry.pre_shutdown().await;
        self.entity_component_behaviour_registry.pre_shutdown().await;
        self.entity_behaviour_registry.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.relation_component_behaviour_manager.shutdown().await;
        self.relation_behaviour_manager.shutdown().await;
        self.entity_component_behaviour_manager.shutdown().await;
        self.entity_behaviour_manager.shutdown().await;
        self.relation_component_behaviour_registry.shutdown().await;
        self.relation_behaviour_registry.shutdown().await;
        self.entity_component_behaviour_registry.shutdown().await;
        self.entity_behaviour_registry.shutdown().await;
    }
}
