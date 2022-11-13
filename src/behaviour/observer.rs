use std::sync::Arc;

use dashmap::DashMap;
use dashmap::DashSet;
use log::trace;
use serde_json::Value;
use uuid::Uuid;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveInstance;
use crate::BehaviourReactiveInstanceContainer;
use crate::BehaviourState;
use crate::PropertyObserverContainer;

pub struct PropertyObserverContainerImpl<T: ReactiveInstance> {
    pub reactive_instance: Arc<T>,
    pub handles: DashMap<String, DashSet<u128>>,
    pub state: BehaviourState,
}

impl<T: ReactiveInstance> PropertyObserverContainerImpl<T> {
    pub fn new(reactive_instance: Arc<T>) -> Self {
        PropertyObserverContainerImpl {
            reactive_instance,
            handles: DashMap::new(),
            state: BehaviourState::Created,
        }
    }
}

impl<T: ReactiveInstance> PropertyObserverContainer for PropertyObserverContainerImpl<T> {
    fn observe_with_handle<F>(&self, name: &str, subscriber: F) -> u128
    where
        F: FnMut(&Value) + 'static,
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
        trace!("Add observer {} {} {}", self.reactive_instance, name, handle_id);
        self.reactive_instance.observe_with_handle(name, subscriber, handle_id);
        handle_id
    }

    fn remove_observer(&self, name: &str, handle_id: u128) {
        self.reactive_instance.remove_observer(name, handle_id);
        if let Some(property_handles) = self.handles.get(name) {
            trace!("Remove observer {} {} {}", self.reactive_instance, property_handles.key(), &handle_id);
            property_handles.remove(&handle_id.clone());
        }
    }

    fn remove_observers(&self, name: &str) {
        if let Some(property_handles) = self.handles.get(name) {
            for handle_id in property_handles.iter() {
                trace!("Remove observer {} {} {}", self.reactive_instance, property_handles.key(), handle_id.key());
                self.reactive_instance.remove_observer(name, *handle_id.key());
            }
        }
        self.handles.remove(name);
    }

    fn remove_all_observers(&self) {
        for property_handles in self.handles.iter() {
            for handle_id in property_handles.iter() {
                trace!("Remove observer {} {} {}", self.reactive_instance, property_handles.key(), handle_id.key());
                self.reactive_instance.remove_observer(property_handles.key(), *handle_id.key());
            }
        }
        self.handles.clear();
    }
}

impl<T: ReactiveInstance> BehaviourReactiveInstanceContainer<T> for PropertyObserverContainerImpl<T> {
    fn get_reactive_instance(&self) -> &Arc<T> {
        &self.reactive_instance
    }
}

impl<T: ReactiveInstance> ReactiveBehaviourContainer for PropertyObserverContainerImpl<T> {
    fn get_behaviours(&self) -> Vec<BehaviourTypeId> {
        self.reactive_instance.get_behaviours()
    }

    fn add_behaviour(&self, ty: BehaviourTypeId) {
        self.reactive_instance.add_behaviour(ty);
    }

    fn remove_behaviour(&self, ty: &BehaviourTypeId) {
        self.reactive_instance.remove_behaviour(ty);
    }

    fn behaves_as(&self, ty: &BehaviourTypeId) -> bool {
        self.reactive_instance.behaves_as(ty)
    }
}
