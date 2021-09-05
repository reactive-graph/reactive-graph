use std::sync::Arc;

use async_trait::async_trait;
use waiter_di::*;

use crate::api::RelationBehaviourManager;
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;
// use crate::registry::relation::*;
use indradb::EdgeKey;
use log::debug;

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
    // TODO: migrate the providers to subsystems
    // connectors_registry: Wrc<dyn ConnectorsRegistry>,
    behaviour_providers: RelationBehaviourProviders,
}

#[async_trait]
#[provides]
impl RelationBehaviourManager for RelationBehaviourManagerImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        // TODO: migrate all registries into subsystems
        // TODO: safety checks
        debug!(
            "RelationBehaviourManager::add_behaviours {}",
            relation_instance.get_key().unwrap().t.0.as_str()
        );
        // self.connectors_registry
        //     .add_behaviours(relation_instance.clone());

        // TODO: unit test with multiple behaviours on a single relation

        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.add_behaviours(relation_instance.clone())
        }
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        // TODO: migrate all registries into subsystems
        // self.connectors_registry
        //     .remove_behaviours(relation_instance.clone());

        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours(relation_instance.clone())
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        // TODO: migrate all registries into subsystems
        // self.connectors_registry
        //     .remove_behaviours_by_key(edge_key.clone());

        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours_by_key(edge_key.clone())
        }
    }

    fn add_provider(&self, provider: Arc<dyn RelationBehaviourProvider>) {
        self.behaviour_providers.0.write().unwrap().push(provider);
    }
}
