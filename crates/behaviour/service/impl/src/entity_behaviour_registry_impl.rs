use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::debug;
use log::warn;
use uuid::Uuid;

use reactive_graph_behaviour_model_api::prelude::*;

use reactive_graph_behaviour_service_api::EntityBehaviourRegistry;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_type_system_api::EntityTypeManager;
use springtime_di::component_alias;
use springtime_di::Component;

#[derive(Component)]
pub struct EntityBehaviourRegistryImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    #[component(default = "DashMap::new")]
    factories: DashMap<EntityBehaviourTypeId, Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>>,
}

#[async_trait]
#[component_alias]
impl EntityBehaviourRegistry for EntityBehaviourRegistryImpl {
    fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>) {
        debug!("Registering entity behaviour {} {}", &entity_behaviour_ty.entity_ty, &entity_behaviour_ty.behaviour_ty);
        if !self.entity_type_manager.has(&entity_behaviour_ty.entity_ty) {
            warn!(
                "Entity behaviour {} is registered on a non-existent entity type {}",
                &entity_behaviour_ty.behaviour_ty, &entity_behaviour_ty.entity_ty
            )
        }
        self.factories.insert(entity_behaviour_ty, factory);
    }

    fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId) {
        debug!("Unregistering entity behaviour {} {}", &entity_behaviour_ty.entity_ty, &entity_behaviour_ty.behaviour_ty);
        self.factories.remove(entity_behaviour_ty);
    }

    fn get_all(&self) -> Vec<EntityBehaviourTypeId> {
        self.factories.iter().map(|f| f.key().clone()).collect()
    }

    fn get(&self, entity_ty: &EntityTypeId) -> Vec<Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>> {
        self.factories
            .iter()
            .filter(|factory| &factory.key().entity_ty == entity_ty)
            .map(|factory| factory.value().clone())
            .collect()
    }

    fn get_factory_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>> {
        self.factories
            .iter()
            .find(|factory| &factory.key().behaviour_ty == behaviour_ty)
            .map(|factory| factory.value().clone())
    }

    fn get_behaviour_types(&self, entity_ty: &EntityTypeId) -> Vec<EntityBehaviourTypeId> {
        self.factories
            .iter()
            .filter(|factory| &factory.key().entity_ty == entity_ty)
            .map(|factory| factory.key().clone())
            .collect()
    }

    fn get_by_behaviour_type(&self, behaviour_ty: &BehaviourTypeId) -> Option<EntityBehaviourTypeId> {
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
impl Lifecycle for EntityBehaviourRegistryImpl {}
