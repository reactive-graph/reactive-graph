use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use inexor_rgf_behaviour_service_api::BehaviourSystem;
use inexor_rgf_behaviour_service_api::EntityBehaviourManager;
use inexor_rgf_behaviour_service_api::EntityBehaviourRegistry;
use inexor_rgf_behaviour_service_api::EntityComponentBehaviourManager;
use inexor_rgf_behaviour_service_api::EntityComponentBehaviourRegistry;
use inexor_rgf_behaviour_service_api::RelationBehaviourManager;
use inexor_rgf_behaviour_service_api::RelationBehaviourRegistry;
use inexor_rgf_behaviour_service_api::RelationComponentBehaviourManager;
use inexor_rgf_behaviour_service_api::RelationComponentBehaviourRegistry;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_type_system_api::TypeSystem;

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

    type_system: Arc<dyn TypeSystem + Send + Sync>,
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

    fn type_system(&self) -> Arc<dyn TypeSystem + Send + Sync> {
        self.type_system.clone()
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
