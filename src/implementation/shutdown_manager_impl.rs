use std::ops::Deref;
use std::sync::{Arc, RwLock};

use crate::builder::ReactiveEntityInstanceBuilder;
use crate::di::*;
use async_trait::async_trait;
use serde_json::json;

use crate::api::{Lifecycle, ReactiveEntityInstanceManager, UUID_SHUTDOWN};
use crate::api::{ShutdownManager, SHUTDOWN};

#[wrapper]
pub struct ShutdownStateContainer(Arc<RwLock<bool>>);

#[provides]
fn create_shutdown_state() -> ShutdownStateContainer {
    ShutdownStateContainer(Arc::new(RwLock::new(false)))
}

#[component]
pub struct ShutdownManagerImpl {
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    shutdown_state: ShutdownStateContainer,
}

#[async_trait]
#[provides]
impl ShutdownManager for ShutdownManagerImpl {
    fn do_shutdown(&self) {
        let mut guard = self.shutdown_state.0.write().unwrap();
        *guard = true;
    }

    fn is_shutdown(&self) -> bool {
        *self.shutdown_state.0.read().unwrap().deref()
    }
}

impl Lifecycle for ShutdownManagerImpl {
    fn init(&self) {
        let entity_instance = ReactiveEntityInstanceBuilder::new(SHUTDOWN)
            .id(UUID_SHUTDOWN)
            .property("label", json!("/org/inexor/system/shutdown"))
            .property(SHUTDOWN, json!(false))
            .get();
        let shutdown_state = self.shutdown_state.0.clone();
        self.reactive_entity_instance_manager.register_reactive_instance(entity_instance.clone());
        entity_instance.properties.get(SHUTDOWN).unwrap().stream.read().unwrap().observe_with_handle(
            move |v| {
                if !v.is_boolean() {
                    return;
                }
                if v.as_bool().unwrap() {
                    let mut guard = shutdown_state.write().unwrap();
                    *guard = true;
                }
            },
            UUID_SHUTDOWN.as_u128(),
        );
    }

    fn shutdown(&self) {
        // Disconnect entity instance
        self.reactive_entity_instance_manager
            .get(UUID_SHUTDOWN)
            .unwrap()
            .properties
            .get(SHUTDOWN)
            .unwrap()
            .stream
            .read()
            .unwrap()
            .remove(UUID_SHUTDOWN.as_u128());
    }
}
