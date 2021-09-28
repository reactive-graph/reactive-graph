use std::sync::Arc;

use async_trait::async_trait;
use waiter_di::*;

use crate::api::RelationBehaviourManager;
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;
use indradb::EdgeKey;
use log::trace;

#[wrapper]
pub struct RelationBehaviourProviders(
    std::sync::RwLock<Vec<std::sync::Arc<dyn RelationBehaviourProvider>>>,
);

#[waiter_di::provides]
fn create_relation_behaviour_providers() -> RelationBehaviourProviders {
    RelationBehaviourProviders(std::sync::RwLock::new(Vec::new()))
}

#[component]
pub struct RelationBehaviourManagerImpl {
    behaviour_providers: RelationBehaviourProviders,
}

#[async_trait]
#[provides]
impl RelationBehaviourManager for RelationBehaviourManagerImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        trace!(
            "RelationBehaviourManager::add_behaviours {}",
            relation_instance.get_key().unwrap().t.0.as_str()
        );
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.add_behaviours(relation_instance.clone())
        }
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours(relation_instance.clone())
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours_by_key(edge_key.clone())
        }
    }

    fn add_provider(&self, provider: Arc<dyn RelationBehaviourProvider>) {
        self.behaviour_providers.0.write().unwrap().push(provider);
    }
}
