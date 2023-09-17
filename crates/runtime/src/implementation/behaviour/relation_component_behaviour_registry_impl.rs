use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::debug;
use log::warn;

use inexor_rgf_behaviour_api::prelude::*;

use crate::api::ComponentManager;
use crate::api::RelationComponentBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::ComponentTypeId;
use crate::model::RelationInstanceId;
use crate::reactive::ReactiveRelation;

#[wrapper]
pub struct RelationComponentBehaviourFactories(
    DashMap<ComponentBehaviourTypeId, Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>>,
);

#[provides]
fn create_entity_component_behaviour_factory_storage() -> RelationComponentBehaviourFactories {
    RelationComponentBehaviourFactories(DashMap::new())
}

#[component]
pub struct RelationComponentBehaviourRegistryImpl {
    component_manager: Wrc<dyn ComponentManager>,

    factories: RelationComponentBehaviourFactories,
}

#[async_trait]
#[provides]
impl RelationComponentBehaviourRegistry for RelationComponentBehaviourRegistryImpl {
    fn register(
        &self,
        component_behaviour_ty: ComponentBehaviourTypeId,
        factory: Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>,
    ) {
        debug!(
            "Registering relation component behaviour {} {}",
            &component_behaviour_ty.component_ty, &component_behaviour_ty.behaviour_ty
        );
        if !self.component_manager.has(&component_behaviour_ty.component_ty) {
            warn!(
                "Relation component behaviour {} is registered on a non-existent component {}",
                &component_behaviour_ty.behaviour_ty, &component_behaviour_ty.component_ty
            )
        }
        self.factories.0.insert(component_behaviour_ty, factory);
    }

    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        debug!(
            "Unregistering relation component behaviour {} {}",
            &component_behaviour_ty.component_ty, &component_behaviour_ty.behaviour_ty
        );
        self.factories.0.remove(component_behaviour_ty);
    }

    fn get_all(&self) -> Vec<ComponentBehaviourTypeId> {
        self.factories.0.iter().map(|f| f.key().clone()).collect()
    }

    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>> {
        self.factories
            .0
            .iter()
            .filter(|factory| &factory.key().component_ty == component_ty)
            .map(|factory| factory.value().clone())
            .collect()
    }

    fn get_behaviour_types(&self, component_ty: &ComponentTypeId) -> Vec<ComponentBehaviourTypeId> {
        self.factories
            .0
            .iter()
            .filter(|factory| &factory.key().component_ty == component_ty)
            .map(|factory| factory.key().clone())
            .collect()
    }

    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<ComponentBehaviourTypeId> {
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
