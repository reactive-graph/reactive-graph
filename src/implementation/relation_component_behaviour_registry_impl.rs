use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;

use crate::api::RelationComponentBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::model::ComponentBehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::model::ReactiveRelationInstance;
use crate::reactive::BehaviourFactory;

#[wrapper]
pub struct RelationComponentBehaviourFactories(DashMap<ComponentBehaviourTypeId, Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>>);

#[provides]
fn create_entity_component_behaviour_factory_storage() -> RelationComponentBehaviourFactories {
    RelationComponentBehaviourFactories(DashMap::new())
}

#[component]
pub struct RelationComponentBehaviourRegistryImpl {
    factories: RelationComponentBehaviourFactories,
}

#[async_trait]
#[provides]
impl RelationComponentBehaviourRegistry for RelationComponentBehaviourRegistryImpl {
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>) {
        self.factories.0.insert(component_behaviour_ty, factory);
    }

    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        self.factories.0.remove(component_behaviour_ty);
    }

    fn get(&self, component_ty: &ComponentTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>> {
        self.factories
            .0
            .iter()
            .filter(|factory| &factory.key().component_ty == component_ty)
            .map(|factory| factory.value().clone())
            .collect()
    }
}
