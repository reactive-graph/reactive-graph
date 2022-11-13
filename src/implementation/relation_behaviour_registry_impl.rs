use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;

use crate::api::RelationBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationBehaviourTypeId;
use crate::model::RelationTypeId;
use crate::reactive::BehaviourFactory;

#[wrapper]
pub struct RelationBehaviourFactories(DashMap<RelationBehaviourTypeId, Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>>);

#[provides]
fn create_relation_behaviour_factory_storage() -> RelationBehaviourFactories {
    RelationBehaviourFactories(DashMap::new())
}

#[component]
pub struct RelationBehaviourRegistryImpl {
    factories: RelationBehaviourFactories,
}

#[async_trait]
#[provides]
impl RelationBehaviourRegistry for RelationBehaviourRegistryImpl {
    fn register(&self, relation_behaviour_ty: RelationBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>) {
        self.factories.0.insert(relation_behaviour_ty, factory);
    }

    fn unregister(&self, relation_behaviour_ty: &RelationBehaviourTypeId) {
        self.factories.0.remove(relation_behaviour_ty);
    }

    fn get(&self, relation_ty: &RelationTypeId) -> Vec<Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>> {
        self.factories
            .0
            .iter()
            .filter(|factory| &factory.key().relation_ty == relation_ty)
            .map(|factory| factory.value().clone())
            .collect()
    }
}
