use async_trait::async_trait;
use std::sync::Arc;

use inexor_rgf_behaviour_api::prelude::*;

use crate::model::RelationInstanceId;
use crate::plugins::RelationComponentBehaviourRegistry;
use crate::reactive::ReactiveRelation;

pub struct RelationComponentBehaviourRegistryImpl {
    relation_component_behaviour_manager: Arc<dyn crate::api::RelationComponentBehaviourManager>,
    relation_component_behaviour_registry: Arc<dyn crate::api::RelationComponentBehaviourRegistry>,
    reactive_relation_manager: Arc<dyn crate::api::ReactiveRelationManager>,
}

impl RelationComponentBehaviourRegistryImpl {
    pub fn new(
        relation_component_behaviour_manager: Arc<dyn crate::api::RelationComponentBehaviourManager>,
        relation_component_behaviour_registry: Arc<dyn crate::api::RelationComponentBehaviourRegistry>,
        reactive_relation_manager: Arc<dyn crate::api::ReactiveRelationManager>,
    ) -> Self {
        Self {
            relation_component_behaviour_manager,
            relation_component_behaviour_registry,
            reactive_relation_manager,
        }
    }
}

#[async_trait]
impl RelationComponentBehaviourRegistry for RelationComponentBehaviourRegistryImpl {
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