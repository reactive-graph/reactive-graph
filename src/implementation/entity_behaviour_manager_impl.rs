use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;
use waiter_di::*;

use crate::api::EntityBehaviourManager;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;
// use crate::registry::entity::*;
use log::debug;

#[wrapper]
pub struct EntityBehaviourProviders(
    std::sync::RwLock<Vec<std::sync::Arc<dyn EntityBehaviourProvider>>>,
);

#[waiter_di::provides]
fn create_behaviour_providers() -> EntityBehaviourProviders {
    EntityBehaviourProviders(std::sync::RwLock::new(Vec::new()))
}

#[component]
pub struct EntityBehaviourManagerImpl {
    // TODO: migrate the providers to subsystems
    // logical_gates_registry: Wrc<dyn LogicalGatesRegistry>,
    // arithmetic_gates_registry: Wrc<dyn ArithmeticGatesRegistry>,
    // numeric_operations_registry: Wrc<dyn NumericOperationsRegistry>,
    // numeric_gates_registry: Wrc<dyn NumericGatesRegistry>,
    // input_gates_registry: Wrc<dyn InputGatesRegistry>,
    // output_gates_registry: Wrc<dyn OutputGatesRegistry>,
    // mqtt_publisher_registry: Wrc<dyn MqttPublisherRegistry>,
    // mqtt_subscriber_registry: Wrc<dyn MqttSubscriberRegistry>,
    // simple_closures_registry: Wrc<dyn SimpleClosuresRegistry>,
    behaviour_providers: EntityBehaviourProviders,
}

#[async_trait]
#[provides]
impl EntityBehaviourManager for EntityBehaviourManagerImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        // TODO: migrate all registries into plugins
        debug!(
            "EntityBehaviourManager::add_behaviours {}",
            entity_instance.id
        );
        // TODO:
        // self.logical_gates_registry
        //     .add_behaviours(entity_instance.clone());
        // self.arithmetic_gates_registry
        //     .add_behaviours(entity_instance.clone());
        // self.numeric_operations_registry
        //     .add_behaviours(entity_instance.clone());
        // self.numeric_gates_registry
        //     .add_behaviours(entity_instance.clone());
        // self.input_gates_registry
        //     .add_behaviours(entity_instance.clone());
        // self.output_gates_registry
        //     .add_behaviours(entity_instance.clone());
        // DONE:
        // self.mqtt_publisher_registry.add_behaviours(entity_instance.clone());
        // self.mqtt_subscriber_registry.add_behaviours(entity_instance.clone());
        // self.simple_closures_registry.add_behaviours(entity_instance.clone());
        // TODO: unit test with multiple behaviours on a single entity

        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.add_behaviours(entity_instance.clone())
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        // TODO: migrate all registries into subsystems
        // TODO:
        // self.logical_gates_registry
        //     .remove_behaviours(entity_instance.clone());
        // self.arithmetic_gates_registry
        //     .remove_behaviours(entity_instance.clone());
        // self.numeric_operations_registry
        //     .remove_behaviours(entity_instance.clone());
        // self.numeric_gates_registry
        //     .remove_behaviours(entity_instance.clone());
        // self.input_gates_registry
        //     .remove_behaviours(entity_instance.clone());
        // self.output_gates_registry
        //     .remove_behaviours(entity_instance.clone());
        // DONE:
        // self.mqtt_publisher_registry.remove_behaviours(entity_instance.clone());
        // self.mqtt_subscriber_registry.remove_behaviours(entity_instance.clone());
        // self.simple_closures_registry.remove_behaviours(entity_instance.clone());

        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours(entity_instance.clone())
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        // TODO: migrate all registries into plugins
        // TODO:
        // self.logical_gates_registry.remove_behaviours_by_id(id);
        // self.arithmetic_gates_registry.remove_behaviours_by_id(id);
        // self.numeric_operations_registry.remove_behaviours_by_id(id);
        // self.numeric_gates_registry.remove_behaviours_by_id(id);
        // self.input_gates_registry.remove_behaviours_by_id(id);
        // self.output_gates_registry.remove_behaviours_by_id(id);
        // DONE:
        // self.mqtt_publisher_registry.remove_behaviours_by_id(id);
        // self.mqtt_subscriber_registry.remove_behaviours_by_id(id);
        // self.simple_closures_registry.remove_behaviours_by_id(id);

        for provider in self.behaviour_providers.0.read().unwrap().iter() {
            provider.remove_behaviours_by_id(id)
        }
    }

    fn add_provider(&self, provider: Arc<dyn EntityBehaviourProvider>) {
        self.behaviour_providers.0.write().unwrap().push(provider);
    }
}
