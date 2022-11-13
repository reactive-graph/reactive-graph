use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::info;

use crate::api::EntityComponentBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::model::ComponentBehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourFactory;

#[wrapper]
pub struct EntityComponentBehaviourFactories(DashMap<ComponentBehaviourTypeId, Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>>);

#[provides]
fn create_entity_component_behaviour_factory_storage() -> EntityComponentBehaviourFactories {
    EntityComponentBehaviourFactories(DashMap::new())
}

#[component]
pub struct EntityComponentBehaviourRegistryImpl {
    factories: EntityComponentBehaviourFactories,
}

#[async_trait]
#[provides]
impl EntityComponentBehaviourRegistry for EntityComponentBehaviourRegistryImpl {
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>) {
        info!(
            "Registering entity component behaviour {} {}",
            &component_behaviour_ty.component_ty, &component_behaviour_ty.behaviour_ty
        );
        self.factories.0.insert(component_behaviour_ty, factory);
    }

    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        info!(
            "Unregistering entity component behaviour {} {}",
            &component_behaviour_ty.component_ty, &component_behaviour_ty.behaviour_ty
        );
        self.factories.0.remove(component_behaviour_ty);
    }

    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>> {
        self.factories
            .0
            .iter()
            .filter(|factory| &factory.key().component_ty == component_ty)
            .map(|factory| factory.value().clone())
            .collect()
    }
}
