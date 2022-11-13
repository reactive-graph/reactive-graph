use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::trace;

use crate::api::RelationComponentBehaviourManager;
use crate::api::RelationComponentBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::ComponentContainer;
use crate::model::ReactiveRelationInstance;
use crate::reactive::RelationBehaviourStorage;

#[wrapper]
pub struct RelationComponentBehaviourStorageWrapper(RelationBehaviourStorage);

#[provides]
fn create_behaviour_providers() -> RelationComponentBehaviourStorageWrapper {
    RelationComponentBehaviourStorageWrapper(RelationBehaviourStorage::new())
}

#[component]
pub struct RelationComponentBehaviourManagerImpl {
    relation_component_behaviour_registry: Wrc<dyn RelationComponentBehaviourRegistry>,

    relation_behaviour_storage: RelationComponentBehaviourStorageWrapper,
}

#[async_trait]
#[provides]
impl RelationComponentBehaviourManager for RelationComponentBehaviourManagerImpl {
    fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        trace!("RelationComponentBehaviourManagerImpl::add_behaviours_to_relation {}", &relation_instance);
        let edge_key = relation_instance.get_key();
        for component_ty in relation_instance.get_components() {
            for factory in self.relation_component_behaviour_registry.get(&component_ty) {
                if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                    let behaviour_ty = behaviour.ty().clone();
                    self.relation_behaviour_storage.0.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                    trace!("Added relation component behaviour {}", &behaviour_ty);
                }
            }
        }
    }

    fn add_behaviours_to_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: crate::model::Component) {
        trace!("RelationComponentBehaviourManagerImpl::add_behaviours_to_relation {}", &relation_instance);
        let edge_key = relation_instance.get_key();
        for factory in self.relation_component_behaviour_registry.get(&component.ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                let behaviour_ty = behaviour.ty().clone();
                self.relation_behaviour_storage.0.insert(edge_key.clone(), behaviour_ty.clone(), behaviour);
                trace!("Added relation component behaviour {}", &behaviour_ty);
            }
        }
    }

    fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.relation_behaviour_storage.0.remove_all(&relation_instance.get_key());
    }

    fn remove_behaviours_from_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: crate::model::Component) {
        let edge_key = relation_instance.get_key();
        for factory in self.relation_component_behaviour_registry.get(&component.ty) {
            self.relation_behaviour_storage.0.remove(&edge_key, factory.behaviour_ty());
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey) {
        self.relation_behaviour_storage.0.remove_all(edge_key);
    }
}
