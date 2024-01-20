use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::debug;
use log::warn;
use springtime_di::component_alias;
use springtime_di::Component;

use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_service_api::RelationComponentBehaviourRegistry;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_model_impl::ReactiveRelation;
use inexor_rgf_type_system_api::ComponentManager;

#[derive(Component)]
pub struct RelationComponentBehaviourRegistryImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    #[component(default = "DashMap::new")]
    factories: DashMap<ComponentBehaviourTypeId, Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>>,
}

#[async_trait]
#[component_alias]
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
        self.factories.insert(component_behaviour_ty, factory);
    }

    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        debug!(
            "Unregistering relation component behaviour {} {}",
            &component_behaviour_ty.component_ty, &component_behaviour_ty.behaviour_ty
        );
        self.factories.remove(component_behaviour_ty);
    }

    fn get_all(&self) -> Vec<ComponentBehaviourTypeId> {
        self.factories.iter().map(|f| f.key().clone()).collect()
    }

    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<RelationInstanceId, ReactiveRelation> + Send + Sync>> {
        self.factories
            .iter()
            .filter(|factory| &factory.key().component_ty == component_ty)
            .map(|factory| factory.value().clone())
            .collect()
    }

    fn get_behaviour_types(&self, component_ty: &ComponentTypeId) -> Vec<ComponentBehaviourTypeId> {
        self.factories
            .iter()
            .filter(|factory| &factory.key().component_ty == component_ty)
            .map(|factory| factory.key().clone())
            .collect()
    }

    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<ComponentBehaviourTypeId> {
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
impl Lifecycle for RelationComponentBehaviourRegistryImpl {}
