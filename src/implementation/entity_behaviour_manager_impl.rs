use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;
use waiter_di::*;

use crate::api::EntityBehaviourManager;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;
use log::debug;

#[wrapper]
pub struct EntityBehaviourProviders(
    std::sync::RwLock<Vec<std::sync::Arc<dyn EntityBehaviourProvider>>>,
);

#[waiter_di::provides]
fn create_behaviour_providers() -> EntityBehaviourProviders {
    EntityBehaviourProviders(std::sync::RwLock::new(Vec::new()))
}

#[component]
pub struct EntityBehaviourManagerImpl {
    behaviour_providers: EntityBehaviourProviders,
}

#[async_trait]
#[provides]
impl EntityBehaviourManager for EntityBehaviourManagerImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        // TODO: migrate all registries into plugins
        debug!(
            "EntityBehaviourManager::add_behaviours {}",
            entity_instance.id
        );
        // TODO: unit test with multiple behaviours on a single entity
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.add_behaviours(entity_instance.clone())
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours(entity_instance.clone())
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours_by_id(id)
        }
    }

    fn add_provider(&self, provider: Arc<dyn EntityBehaviourProvider>) {
        self.behaviour_providers.0.write().unwrap().push(provider);
    }
}
