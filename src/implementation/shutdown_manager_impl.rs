use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;
use std::time;

use async_trait::async_trait;
use inexor_rgf_core_model::ComponentTypeId;
use inexor_rgf_core_model::EntityTypeId;
use serde_json::json;
use tokio::task;

use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ShutdownManager;
use crate::api::UUID_SHUTDOWN;
use crate::api::UUID_SHUTDOWN_TRIGGER;
use crate::builder::EntityTypeBuilder;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::di::*;
use crate::implementation::COMPONENT_ACTION;
use crate::implementation::COMPONENT_LABELED;
use crate::implementation::NAMESPACE_CORE;
use crate::implementation::NAMESPACE_LOGICAL;
use crate::implementation::PROPERTY_LABEL;
use crate::implementation::PROPERTY_SHUTDOWN;
use crate::implementation::PROPERTY_TRIGGER;
use crate::implementation::TYPE_SHUTDOWN;
use crate::model::ReactivePropertyContainer;

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
        let ty = EntityTypeId::new_from_type(NAMESPACE_CORE, TYPE_SHUTDOWN);
        let _ = self.entity_type_manager.register(EntityTypeBuilder::new(&ty).build());
        let shutdown_handler = ReactiveEntityInstanceBuilder::new(ty)
            .id(UUID_SHUTDOWN)
            .property(PROPERTY_LABEL, json!("/org/inexor/system/shutdown"))
            .property(PROPERTY_SHUTDOWN, json!(false))
            .property(PROPERTY_TRIGGER, json!(false))
            .component(ComponentTypeId::new_from_type(NAMESPACE_CORE, COMPONENT_LABELED))
            .component(ComponentTypeId::new_from_type(NAMESPACE_LOGICAL, COMPONENT_ACTION))
            .build();
        let _ = self.reactive_entity_instance_manager.register_reactive_instance(shutdown_handler.clone());
        let shutdown_state = self.shutdown_state.0.clone();
        shutdown_handler.observe_with_handle(
            PROPERTY_TRIGGER,
            move |v| {
                if v.is_boolean() && v.as_bool().unwrap() {
                    let mut guard = shutdown_state.write().unwrap();
                    *guard = true;
                }
            },
            UUID_SHUTDOWN_TRIGGER.as_u128(),
        );
        let shutdown_state = self.shutdown_state.0.clone();
        shutdown_handler.observe_with_handle(
            PROPERTY_SHUTDOWN,
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
        // Disconnect reactive streams of the shutdown handler
        if let Some(shutdown_handler) = self.reactive_entity_instance_manager.get(UUID_SHUTDOWN) {
            shutdown_handler.remove_observer(PROPERTY_TRIGGER, UUID_SHUTDOWN_TRIGGER.as_u128());
            shutdown_handler.remove_observer(PROPERTY_SHUTDOWN, UUID_SHUTDOWN.as_u128());
        }
    }
}