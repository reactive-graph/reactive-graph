use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::trace;

use crate::api::RelationBehaviourManager;
use crate::api::RelationBehaviourRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::ReactiveRelationInstance;
use crate::reactive::RelationBehaviourStorage;

#[wrapper]
pub struct RelationBehaviourStorageWrapper(RelationBehaviourStorage);

#[provides]
fn create_relation_behaviour_providers() -> RelationBehaviourStorageWrapper {
    RelationBehaviourStorageWrapper(RelationBehaviourStorage::new())
}

#[component]
pub struct RelationBehaviourManagerImpl {
    relation_behaviour_registry: Wrc<dyn RelationBehaviourRegistry>,

    relation_behaviour_storage: RelationBehaviourStorageWrapper,
}

#[async_trait]
#[provides]
impl RelationBehaviourManager for RelationBehaviourManagerImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        trace!("RelationBehaviourManager::add_behaviours {}", relation_instance);
        let relation_ty = relation_instance.relation_type_id();
        for factory in self.relation_behaviour_registry.get(&relation_ty) {
            if let Ok(behaviour) = factory.create(relation_instance.clone()) {
                self.relation_behaviour_storage.0.insert(edge_key.clone(), behaviour.ty().clone(), behaviour);
            }
        }
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.relation_behaviour_storage.0.remove_all(&relation_instance.get_key());
    }

    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey) {
        self.relation_behaviour_storage.0.remove_all(&edge_key);
    }
}
