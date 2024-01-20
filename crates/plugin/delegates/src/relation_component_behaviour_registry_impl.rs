use std::sync::Arc;

use async_trait::async_trait;

use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_reactive_model_impl::ReactiveRelation;

pub struct RelationComponentBehaviourRegistryDelegate {
    relation_component_behaviour_manager: Arc<dyn inexor_rgf_behaviour_service_api::RelationComponentBehaviourManager + Send + Sync>,
    relation_component_behaviour_registry: Arc<dyn inexor_rgf_behaviour_service_api::RelationComponentBehaviourRegistry + Send + Sync>,
    reactive_relation_manager: Arc<dyn inexor_rgf_reactive_service_api::ReactiveRelationManager + Send + Sync>,
}

impl RelationComponentBehaviourRegistryDelegate {
    pub fn new(
        relation_component_behaviour_manager: Arc<dyn inexor_rgf_behaviour_service_api::RelationComponentBehaviourManager + Send + Sync>,
        relation_component_behaviour_registry: Arc<dyn inexor_rgf_behaviour_service_api::RelationComponentBehaviourRegistry + Send + Sync>,
        reactive_relation_manager: Arc<dyn inexor_rgf_reactive_service_api::ReactiveRelationManager + Send + Sync>,
    ) -> Self {
        Self {
            relation_component_behaviour_manager,
            relation_component_behaviour_registry,
            reactive_relation_manager,
        }
    }
}

#[async_trait]
impl inexor_rgf_plugin_api::RelationComponentBehaviourRegistry for RelationComponentBehaviourRegistryDelegate {
    async fn register(
        &self,
        component_behaviour_ty: ComponentBehaviourTypeId,
        factory: Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>,
    ) {
        self.relation_component_behaviour_registry.register(component_behaviour_ty.clone(), factory);
        self.reactive_relation_manager.add_behaviour_to_all_relation_components(&component_behaviour_ty);
    }

    async fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        self.relation_component_behaviour_registry.unregister(component_behaviour_ty);
        self.relation_component_behaviour_manager
            .remove_behaviours_by_behaviour(&component_behaviour_ty.behaviour_ty);
    }
}
