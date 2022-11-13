use std::sync::Arc;

use async_trait::async_trait;
use log::trace;
use uuid::Uuid;

use crate::api::EntityBehaviourManager;
use crate::api::EntityBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::ReactiveEntityInstance;
use crate::reactive::EntityBehaviourStorage;

#[wrapper]
pub struct EntityBehaviourStorageWrapper(EntityBehaviourStorage);

#[provides]
fn create_entity_behaviour_storage() -> EntityBehaviourStorageWrapper {
    EntityBehaviourStorageWrapper(EntityBehaviourStorage::new())
}

#[component]
pub struct EntityBehaviourManagerImpl {
    entity_behaviour_registry: Wrc<dyn EntityBehaviourRegistry>,

    entity_behaviour_storage: EntityBehaviourStorageWrapper,
}

#[async_trait]
#[provides]
impl EntityBehaviourManager for EntityBehaviourManagerImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        trace!("EntityBehaviourManager::add_behaviours {}", &entity_instance);
        for factory in self.entity_behaviour_registry.get(&entity_instance.ty) {
            if let Ok(behaviour) = factory.create(entity_instance.clone()) {
                self.entity_behaviour_storage.0.insert(entity_instance.id, behaviour.ty().clone(), behaviour);
            }
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        trace!("EntityBehaviourManager::remove_behaviours {}", &entity_instance);
        self.entity_behaviour_storage.0.remove_all(&entity_instance.id);
    }

    fn remove_behaviours_by_id(&self, id: &Uuid) {
        trace!("EntityBehaviourManager::remove_behaviours {}", &id);
        self.entity_behaviour_storage.0.remove_all(id);
    }
}
