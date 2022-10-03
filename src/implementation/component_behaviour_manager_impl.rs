use std::sync::{Arc, RwLock};

use crate::di::{component, provides, wrapper, Component};
use async_trait::async_trait;
use uuid::Uuid;

use crate::api::ComponentBehaviourManager;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;
use crate::plugins::ComponentBehaviourProvider;
use indradb::EdgeKey;
use log::trace;

#[wrapper]
pub struct ComponentBehaviourProviders(RwLock<Vec<Arc<dyn ComponentBehaviourProvider>>>);

#[provides]
fn create_behaviour_providers() -> ComponentBehaviourProviders {
    ComponentBehaviourProviders(RwLock::new(Vec::new()))
}

#[component]
pub struct ComponentBehaviourManagerImpl {
    behaviour_providers: ComponentBehaviourProviders,
}

#[async_trait]
#[provides]
impl ComponentBehaviourManager for ComponentBehaviourManagerImpl {
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        trace!("ComponentBehaviourManager::add_behaviours_to_entity {}", entity_instance.id);
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.add_behaviours_to_entity(entity_instance.clone())
        }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        trace!("ComponentBehaviourManager::add_behaviours_to_entity {}", entity_instance.id);
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.add_behaviours_to_entity_component(entity_instance.clone(), component.clone())
        }
    }

    fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        trace!("ComponentBehaviourManager::add_behaviours_to_relation {}", relation_instance.get_key().unwrap().t.to_string());
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.add_behaviours_to_relation(relation_instance.clone())
        }
    }

    fn add_behaviours_to_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: crate::model::Component) {
        trace!("ComponentBehaviourManager::add_behaviours_to_relation {}", relation_instance.get_key().unwrap().t.to_string());
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.add_behaviours_to_relation_component(relation_instance.clone(), component.clone())
        }
    }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours_from_entity(entity_instance.clone())
        }
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours_from_entity_component(entity_instance.clone(), component.clone())
        }
    }

    fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours_from_relation(relation_instance.clone())
        }
    }

    fn remove_behaviours_from_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: crate::model::Component) {
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours_from_relation_component(relation_instance.clone(), component.clone())
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours_by_id(id)
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours_by_key(edge_key.clone())
        }
    }

    fn add_provider(&self, provider: Arc<dyn ComponentBehaviourProvider>) {
        self.behaviour_providers.0.write().unwrap().push(provider);
    }
}
