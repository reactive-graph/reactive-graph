use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;

use crate::api::EntityBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::model::EntityBehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourFactory;

#[wrapper]
pub struct EntityBehaviourFactories(DashMap<EntityBehaviourTypeId, Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>>);

#[provides]
fn create_entity_behaviour_factory_storage() -> EntityBehaviourFactories {
    EntityBehaviourFactories(DashMap::new())
}

#[component]
pub struct EntityBehaviourRegistryImpl {
    factories: EntityBehaviourFactories,
}

#[async_trait]
#[provides]
impl EntityBehaviourRegistry for EntityBehaviourRegistryImpl {
    fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>) {
        self.factories.0.insert(entity_behaviour_ty, factory);
    }

    fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId) {
        self.factories.0.remove(entity_behaviour_ty);
    }

    fn get(&self, entity_ty: &EntityTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>> {
        self.factories
            .0
            .iter()
            .filter(|factory| &factory.key().entity_ty == entity_ty)
            .map(|factory| factory.value().clone())
            .collect()
    }
}
