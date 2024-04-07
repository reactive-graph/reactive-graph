use dashmap::DashMap;
use dashmap::DashSet;
use log::trace;
use serde_json::Value;
use uuid::Uuid;

use crate::PropertyObserverContainer;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_reactive_model_impl::ReactiveEntity;

/// Container which manages the observers of a reactive entity instance.
pub struct EntityPropertyObserverContainerImpl {
    pub reactive_instance: ReactiveEntity,
    pub handles: DashMap<String, DashSet<u128>>,
}

impl EntityPropertyObserverContainerImpl {
    /// Creates a property observer container for the given reactive entity instance.
    pub fn new(reactive_instance: ReactiveEntity) -> Self {
        EntityPropertyObserverContainerImpl {
            reactive_instance,
            handles: DashMap::new(),
        }
    }
}

impl PropertyObserverContainer for EntityPropertyObserverContainerImpl {
    fn observe_with_handle<F>(&self, name: &str, subscriber: F) -> u128
    where
        F: FnMut(&Value) + 'static + Send,
    {
        let handle_id = Uuid::new_v4().as_u128();
        match self.handles.get(name) {
            None => {
                let property_handles = DashSet::new();
                property_handles.insert(handle_id);
                self.handles.insert(name.to_string(), property_handles);
            }
            Some(property_handles) => {
                property_handles.insert(handle_id);
            }
        }
        trace!("Adding observer {} {} {}", self.reactive_instance, name, handle_id);
        self.reactive_instance.observe_with_handle(name, subscriber, handle_id);
        handle_id
    }

    fn propagate(&self, name: &str, target_property_name: &str) {
        let reactive_instance = self.reactive_instance.clone();
        let target_property_name = target_property_name.to_string();
        self.observe_with_handle(name, move |value| {
            reactive_instance.set(&target_property_name, value.clone());
        });
    }

    fn remove_observer(&self, name: &str, handle_id: u128) {
        self.reactive_instance.remove_observer(name, handle_id);
        if let Some(property_handles) = self.handles.get(name) {
            trace!("Removing observer {} {} {}", self.reactive_instance, property_handles.key(), &handle_id);
            property_handles.remove(&handle_id.clone());
        }
    }

    fn remove_observers(&self, name: &str) {
        if let Some(property_handles) = self.handles.get(name) {
            for handle_id in property_handles.iter() {
                trace!("Removing observer {} {} {}", self.reactive_instance, property_handles.key(), handle_id.key());
                self.reactive_instance.remove_observer(name, *handle_id.key());
            }
        }
        self.handles.remove(name);
    }

    fn remove_all_observers(&self) {
        for property_handles in self.handles.iter() {
            for handle_id in property_handles.iter() {
                trace!("Removing observer {} {} {}", self.reactive_instance, property_handles.key(), handle_id.key());
                self.reactive_instance.remove_observer(property_handles.key(), *handle_id.key());
            }
        }
        self.handles.clear();
    }
}
