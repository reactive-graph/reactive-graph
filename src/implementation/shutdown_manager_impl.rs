use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::time;

use crate::builder::EntityTypeBuilder;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::di::*;
use async_trait::async_trait;
use serde_json::json;
use tokio::task;

use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ShutdownManager;
use crate::api::LABEL;
use crate::api::SHUTDOWN;
use crate::api::TRIGGER;
use crate::api::UUID_SHUTDOWN;
use crate::api::UUID_SHUTDOWN_TRIGGER;

#[wrapper]
pub struct ShutdownStateContainer(Arc<RwLock<bool>>);

#[provides]
fn create_shutdown_state() -> ShutdownStateContainer {
    ShutdownStateContainer(Arc::new(RwLock::new(false)))
}

#[component]
pub struct ShutdownManagerImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,
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
        let _ = self.entity_type_manager.register(EntityTypeBuilder::new("core", SHUTDOWN).build());
        let entity_instance = ReactiveEntityInstanceBuilder::new("core", SHUTDOWN)
            .id(UUID_SHUTDOWN)
            .property(LABEL, json!("/org/inexor/system/shutdown"))
            .property(SHUTDOWN, json!(false))
            .property(TRIGGER, json!(false))
            .build();
        entity_instance.components.insert("labeled".to_owned());
        entity_instance.components.insert("action".to_owned());
        self.reactive_entity_instance_manager.register_reactive_instance(entity_instance.clone());
        let shutdown_state = self.shutdown_state.0.clone();
        entity_instance.properties.get(TRIGGER).unwrap().stream.read().unwrap().observe_with_handle(
            move |v| {
                if v.is_boolean() && v.as_bool().unwrap() {
                    let mut guard = shutdown_state.write().unwrap();
                    *guard = true;
                }
            },
            UUID_SHUTDOWN_TRIGGER.as_u128(),
        );
        let shutdown_state = self.shutdown_state.0.clone();
        entity_instance.properties.get(SHUTDOWN).unwrap().stream.read().unwrap().observe_with_handle(
            move |v| {
                if v.is_boolean() && v.as_bool().unwrap() {
                    let mut guard = shutdown_state.write().unwrap();
                    *guard = true;
                }
                if v.is_number() {
                    let shutdown_in_seconds = time::Duration::from_secs(v.as_u64().unwrap());
                    let shutdown_state_deferred = shutdown_state.clone();
                    task::spawn(async move {
                        tokio::time::sleep(shutdown_in_seconds).await;
                        let mut guard = shutdown_state_deferred.write().unwrap();
                        *guard = true;
                    });
                }
            },
            UUID_SHUTDOWN.as_u128(),
        );
    }

    fn shutdown(&self) {
        // Disconnect entity instances
        self.reactive_entity_instance_manager
            .get(UUID_SHUTDOWN)
            .unwrap()
            .properties
            .get(TRIGGER)
            .unwrap()
            .stream
            .read()
            .unwrap()
            .remove(UUID_SHUTDOWN_TRIGGER.as_u128());
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
