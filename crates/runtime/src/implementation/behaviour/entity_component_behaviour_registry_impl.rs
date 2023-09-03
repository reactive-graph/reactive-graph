use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use inexor_rgf_core_di::Wrc;
use log::debug;
use log::warn;
use uuid::Uuid;

use crate::api::ComponentManager;
use crate::api::EntityComponentBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::reactive::BehaviourTypeId;
use crate::reactive::ComponentBehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::reactive::ReactiveEntity;
use crate::behaviour::BehaviourFactory;

#[wrapper]
pub struct EntityComponentBehaviourFactories(DashMap<ComponentBehaviourTypeId, Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>>);

#[provides]
fn create_entity_component_behaviour_factory_storage() -> EntityComponentBehaviourFactories {
    EntityComponentBehaviourFactories(DashMap::new())
}

#[component]
pub struct EntityComponentBehaviourRegistryImpl {
    component_manager: Wrc<dyn ComponentManager>,

    factories: EntityComponentBehaviourFactories,
}

#[async_trait]
#[provides]
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
        self.factories.0.insert(component_behaviour_ty, factory);
    }

    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        debug!(
            "Unregistering entity component behaviour {} {}",
            &component_behaviour_ty.component_ty, &component_behaviour_ty.behaviour_ty
        );
        self.factories.0.remove(component_behaviour_ty);
    }

    fn get_all(&self) -> Vec<ComponentBehaviourTypeId> {
        self.factories.0.iter().map(|f| f.key().clone()).collect()
    }

    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>> {
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
