use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::debug;
use log::warn;
use springtime_di::Component;
use springtime_di::component_alias;
use uuid::Uuid;

use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourRegistry;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_type_system_api::ComponentManager;

#[derive(Component)]
pub struct EntityComponentBehaviourRegistryImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    #[component(default = "DashMap::new")]
    factories: DashMap<ComponentBehaviourTypeId, Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>>,
}

#[async_trait]
#[component_alias]
impl EntityComponentBehaviourRegistry for EntityComponentBehaviourRegistryImpl {
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>) {
        debug!(
            "Registering entity component behaviour {} {}",
            &component_behaviour_ty.component_ty, &component_behaviour_ty.behaviour_ty
        );
        if !self.component_manager.has(&component_behaviour_ty.component_ty) {
            warn!(
                "Entity component behaviour {} is registered on a non-existent component {}",
                &component_behaviour_ty.behaviour_ty, &component_behaviour_ty.component_ty
            )
        }
        self.factories.insert(component_behaviour_ty, factory);
    }

    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        debug!(
            "Unregistering entity component behaviour {} {}",
            &component_behaviour_ty.component_ty, &component_behaviour_ty.behaviour_ty
        );
        self.factories.remove(component_behaviour_ty);
    }

    fn get_all(&self) -> Vec<ComponentBehaviourTypeId> {
        self.factories.iter().map(|f| f.key().clone()).collect()
    }

    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>> {
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
impl Lifecycle for EntityComponentBehaviourRegistryImpl {}
