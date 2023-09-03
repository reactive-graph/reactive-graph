use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::debug;
use log::warn;
use uuid::Uuid;

use crate::api::EntityBehaviourRegistry;
use crate::api::EntityTypeManager;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::reactive::BehaviourTypeId;
use crate::reactive::EntityBehaviourTypeId;
use crate::model::EntityTypeId;
use crate::reactive::ReactiveEntity;
use crate::behaviour::BehaviourFactory;

#[wrapper]
pub struct EntityBehaviourFactories(DashMap<EntityBehaviourTypeId, Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>>);

#[provides]
fn create_entity_behaviour_factory_storage() -> EntityBehaviourFactories {
    EntityBehaviourFactories(DashMap::new())
}

#[component]
pub struct EntityBehaviourRegistryImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,

    factories: EntityBehaviourFactories,
}

#[async_trait]
#[provides]
impl EntityBehaviourRegistry for EntityBehaviourRegistryImpl {
    fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>) {
        debug!("Registering entity behaviour {} {}", &entity_behaviour_ty.entity_ty, &entity_behaviour_ty.behaviour_ty);
        if !self.entity_type_manager.has(&entity_behaviour_ty.entity_ty) {
            warn!(
                "Entity behaviour {} is registered on a non-existent entity type {}",
                &entity_behaviour_ty.behaviour_ty, &entity_behaviour_ty.entity_ty
            )
        }
        self.factories.0.insert(entity_behaviour_ty, factory);
    }

    fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId) {
        debug!("Unregistering entity behaviour {} {}", &entity_behaviour_ty.entity_ty, &entity_behaviour_ty.behaviour_ty);
        self.factories.0.remove(entity_behaviour_ty);
    }

    fn get_all(&self) -> Vec<EntityBehaviourTypeId> {
        self.factories.0.iter().map(|f| f.key().clone()).collect()
    }

    fn get(&self, entity_ty: &EntityTypeId) -> Vec<Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>> {
        self.factories
            .0
            .iter()
            .filter(|factory| &factory.key().entity_ty == entity_ty)
            .map(|factory| factory.value().clone())
            .collect()
    }

    fn get_factory_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>> {
        self.factories
            .0
            .iter()
            .find(|factory| &factory.key().behaviour_ty == behaviour_ty)
            .map(|factory| factory.value().clone())
    }

    fn get_behaviour_types(&self, entity_ty: &EntityTypeId) -> Vec<EntityBehaviourTypeId> {
        self.factories
            .0
            .iter()
            .filter(|factory| &factory.key().entity_ty == entity_ty)
            .map(|factory| factory.key().clone())
            .collect()
    }

    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<EntityBehaviourTypeId> {
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
