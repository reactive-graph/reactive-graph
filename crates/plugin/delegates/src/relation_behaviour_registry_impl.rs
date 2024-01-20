use std::sync::Arc;

use async_trait::async_trait;

use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_reactive_model_impl::ReactiveRelation;

pub struct RelationBehaviourRegistryDelegate {
    relation_behaviour_manager: Arc<dyn inexor_rgf_behaviour_service_api::RelationBehaviourManager + Send + Sync>,
    relation_behaviour_registry: Arc<dyn inexor_rgf_behaviour_service_api::RelationBehaviourRegistry + Send + Sync>,
    reactive_relation_manager: Arc<dyn inexor_rgf_reactive_service_api::ReactiveRelationManager + Send + Sync>,
}

impl RelationBehaviourRegistryDelegate {
    pub fn new(
        relation_behaviour_manager: Arc<dyn inexor_rgf_behaviour_service_api::RelationBehaviourManager + Send + Sync>,
        relation_behaviour_registry: Arc<dyn inexor_rgf_behaviour_service_api::RelationBehaviourRegistry + Send + Sync>,
        reactive_relation_manager: Arc<dyn inexor_rgf_reactive_service_api::ReactiveRelationManager + Send + Sync>,
    ) -> Self {
        Self {
            relation_behaviour_manager,
            relation_behaviour_registry,
            reactive_relation_manager,
        }
    }
}

#[async_trait]
impl inexor_rgf_plugin_api::RelationBehaviourRegistry for RelationBehaviourRegistryDelegate {
    async fn register(
        &self,
        relation_behaviour_ty: RelationBehaviourTypeId,
        factory: Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>,
    ) {
        self.relation_behaviour_registry.register(relation_behaviour_ty.clone(), factory);
        self.reactive_relation_manager.add_behaviour_to_all_relation_instances(&relation_behaviour_ty);
    }

    async fn unregister(&self, relation_behaviour_ty: &RelationBehaviourTypeId) {
        self.relation_behaviour_registry.unregister(relation_behaviour_ty);
        self.relation_behaviour_manager
            .remove_behaviours_by_behaviour(&relation_behaviour_ty.behaviour_ty);
    }
}
