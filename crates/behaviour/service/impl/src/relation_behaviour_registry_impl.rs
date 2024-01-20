use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::debug;
use log::warn;
use springtime_di::component_alias;
use springtime_di::Component;

use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_service_api::RelationBehaviourRegistry;
use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_model_impl::ReactiveRelation;
use inexor_rgf_type_system_api::RelationTypeManager;

#[derive(Component)]
pub struct RelationBehaviourRegistryImpl {
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,

    #[component(default = "DashMap::new")]
    factories: DashMap<RelationBehaviourTypeId, Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>>,
}

#[async_trait]
#[component_alias]
impl RelationBehaviourRegistry for RelationBehaviourRegistryImpl {
    fn register(&self, relation_behaviour_ty: RelationBehaviourTypeId, factory: Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>) {
        debug!(
            "Registering relation behaviour {} {}",
            &relation_behaviour_ty.relation_ty, &relation_behaviour_ty.behaviour_ty
        );
        if !self.relation_type_manager.has(&relation_behaviour_ty.relation_ty) {
            warn!(
                "Relation behaviour {} is registered on a non-existent relation type {}",
                &relation_behaviour_ty.behaviour_ty, &relation_behaviour_ty.relation_ty
            )
        }
        self.factories.insert(relation_behaviour_ty, factory);
    }

    fn unregister(&self, relation_behaviour_ty: &RelationBehaviourTypeId) {
        debug!(
            "Unregistering relation behaviour {} {}",
            &relation_behaviour_ty.relation_ty, &relation_behaviour_ty.behaviour_ty
        );
        self.factories.remove(relation_behaviour_ty);
    }

    fn get_all(&self) -> Vec<RelationBehaviourTypeId> {
        self.factories.iter().map(|f| f.key().clone()).collect()
    }

    fn get(&self, relation_ty: &RelationTypeId) -> Vec<Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>> {
        self.factories
            .iter()
            .filter(|factory| &factory.key().relation_ty == relation_ty)
            .map(|factory| factory.value().clone())
            .collect()
    }

    fn get_factory_by_behaviour_type(
        &self,
        behaviour_ty: &BehaviourTypeId,
    ) -> Option<Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>> {
        self.factories
            .iter()
            .find(|factory| &factory.key().behaviour_ty == behaviour_ty)
            .map(|factory| factory.value().clone())
    }

    fn get_behaviour_types(&self, relation_ty: &RelationTypeId) -> Vec<RelationBehaviourTypeId> {
        self.factories
            .iter()
            .filter(|factory| &factory.key().relation_ty == relation_ty)
            .map(|factory| factory.key().clone())
            .collect()
    }

    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<RelationBehaviourTypeId> {
        self.factories
            .iter()
            .find(|factory| &factory.key().behaviour_ty == behaviour_ty)
            .map(|factory| factory.key().clone())
    }

    fn count(&self) -> usize {
        self.factories.len()
    }
}

#[async_trait]
impl Lifecycle for RelationBehaviourRegistryImpl {}
