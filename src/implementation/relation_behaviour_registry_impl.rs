use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::debug;
use log::warn;

use crate::api::RelationBehaviourRegistry;
use crate::api::RelationTypeManager;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::BehaviourTypeId;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationBehaviourTypeId;
use crate::model::RelationTypeId;
use crate::reactive::BehaviourFactory;

#[wrapper]
pub struct RelationBehaviourFactories(DashMap<RelationBehaviourTypeId, Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>>);

#[provides]
fn create_relation_behaviour_factory_storage() -> RelationBehaviourFactories {
    RelationBehaviourFactories(DashMap::new())
}

#[component]
pub struct RelationBehaviourRegistryImpl {
    relation_type_manager: Wrc<dyn RelationTypeManager>,

    factories: RelationBehaviourFactories,
}

#[async_trait]
#[provides]
impl RelationBehaviourRegistry for RelationBehaviourRegistryImpl {
    fn register(&self, relation_behaviour_ty: RelationBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>) {
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
        self.factories.0.insert(relation_behaviour_ty, factory);
    }

    fn unregister(&self, relation_behaviour_ty: &RelationBehaviourTypeId) {
        debug!(
            "Unregistering relation behaviour {} {}",
            &relation_behaviour_ty.relation_ty, &relation_behaviour_ty.behaviour_ty
        );
        self.factories.0.remove(relation_behaviour_ty);
    }

    fn get_all(&self) -> Vec<RelationBehaviourTypeId> {
        self.factories.0.iter().map(|f| f.key().clone()).collect()
    }

    fn get(&self, relation_ty: &RelationTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>> {
        self.factories
            .0
            .iter()
            .filter(|factory| &factory.key().relation_ty == relation_ty)
            .map(|factory| factory.value().clone())
            .collect()
    }

    fn get_factory_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>> {
        self.factories
            .0
            .iter()
            .find(|factory| &factory.key().behaviour_ty == behaviour_ty)
            .map(|factory| factory.value().clone())
    }

    fn get_behaviour_types(&self, relation_ty: &RelationTypeId) -> Vec<RelationBehaviourTypeId> {
        self.factories
            .0
            .iter()
            .filter(|factory| &factory.key().relation_ty == relation_ty)
            .map(|factory| factory.key().clone())
            .collect()
    }

    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<RelationBehaviourTypeId> {
        self.factories
            .0
            .iter()
            .find(|factory| &factory.key().behaviour_ty == behaviour_ty)
            .map(|factory| factory.key().clone())
    }

    fn count(&self) -> usize {
        self.factories.0.len()
    }
}
