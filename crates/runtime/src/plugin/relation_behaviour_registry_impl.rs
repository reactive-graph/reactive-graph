use async_trait::async_trait;
use std::sync::Arc;

use inexor_rgf_behaviour_api::prelude::*;

use crate::model::RelationInstanceId;
use crate::plugins::RelationBehaviourRegistry;
use crate::reactive::ReactiveRelation;

pub struct RelationBehaviourRegistryImpl {
    relation_behaviour_manager: Arc<dyn crate::api::RelationBehaviourManager>,
    relation_behaviour_registry: Arc<dyn crate::api::RelationBehaviourRegistry>,
    reactive_relation_manager: Arc<dyn crate::api::ReactiveRelationManager>,
}

impl RelationBehaviourRegistryImpl {
    pub fn new(
        relation_behaviour_manager: Arc<dyn crate::api::RelationBehaviourManager>,
        relation_behaviour_registry: Arc<dyn crate::api::RelationBehaviourRegistry>,
        reactive_relation_manager: Arc<dyn crate::api::ReactiveRelationManager>,
    ) -> Self {
        Self {
            relation_behaviour_manager,
            relation_behaviour_registry,
            reactive_relation_manager,
        }
    }
}

#[async_trait]
impl RelationBehaviourRegistry for RelationBehaviourRegistryImpl {
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
