use std::sync::Arc;

use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use async_trait::async_trait;
use dashmap::DashMap;
use uuid::Uuid;

use crate::api::EntityBehaviourManager;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;
use log::trace;

#[wrapper]
pub struct EntityBehaviourProviders(DashMap<Uuid, Arc<dyn EntityBehaviourProvider>>);

#[provides]
fn create_behaviour_providers() -> EntityBehaviourProviders {
    EntityBehaviourProviders(DashMap::new())
}

#[component]
pub struct EntityBehaviourManagerImpl {
    behaviour_providers: EntityBehaviourProviders,
}

#[async_trait]
#[provides]
impl EntityBehaviourManager for EntityBehaviourManagerImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        trace!("EntityBehaviourManager::add_behaviours {}", entity_instance.id);
        for provider in self.behaviour_providers.0.iter() {
            provider.add_behaviours(entity_instance.clone())
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        for provider in self.behaviour_providers.0.iter() {
            provider.remove_behaviours(entity_instance.clone())
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        for provider in self.behaviour_providers.0.iter() {
            provider.remove_behaviours_by_id(id)
        }
    }

    fn add_provider(&self, id: Uuid, provider: Arc<dyn EntityBehaviourProvider>) {
        self.behaviour_providers.0.insert(id, provider);
    }

    fn remove_provider(&self, id: &Uuid) {
        self.behaviour_providers.0.remove(id);
    }
}
