use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use indradb::EdgeKey;
use log::trace;
use uuid::Uuid;

use crate::api::RelationBehaviourManager;
use crate::di::*;
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;

#[wrapper]
pub struct RelationBehaviourProviders(DashMap<Uuid, Arc<dyn RelationBehaviourProvider>>);

#[provides]
fn create_relation_behaviour_providers() -> RelationBehaviourProviders {
    RelationBehaviourProviders(DashMap::new())
}

#[component]
pub struct RelationBehaviourManagerImpl {
    behaviour_providers: RelationBehaviourProviders,
}

#[async_trait]
#[provides]
impl RelationBehaviourManager for RelationBehaviourManagerImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        trace!("RelationBehaviourManager::add_behaviours {}", relation_instance.get_key().t.to_string());
        for provider in self.behaviour_providers.0.iter() {
            provider.add_behaviours(relation_instance.clone())
        }
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        for provider in self.behaviour_providers.0.iter() {
            provider.remove_behaviours(relation_instance.clone())
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey) {
        for provider in self.behaviour_providers.0.iter() {
            provider.remove_behaviours_by_key(&edge_key)
        }
    }

    fn add_provider(&self, id: Uuid, provider: Arc<dyn RelationBehaviourProvider>) {
        self.behaviour_providers.0.insert(id, provider);
    }

    fn remove_provider(&self, id: &Uuid) {
        self.behaviour_providers.0.remove(id);
    }
}
