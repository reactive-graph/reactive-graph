use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use serde_json::json;

use crate::api::Lifecycle;
use crate::api::SystemEvent;
use crate::api::SystemEventManager;
use crate::api::SystemEventTypes;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::di::*;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;

#[wrapper]
pub struct SystemEventInstanceStorage(RwLock<HashMap<SystemEventTypes, Arc<ReactiveEntityInstance>>>);

#[provides]
fn create_shutdown_state() -> SystemEventInstanceStorage {
    SystemEventInstanceStorage(RwLock::new(HashMap::new()))
}

#[component]
pub struct SystemEventManagerImpl {
    system_event_instances: SystemEventInstanceStorage,
}

#[async_trait]
#[provides]
impl SystemEventManager for SystemEventManagerImpl {
    fn emit_event(&self, event: SystemEvent) {
        let writer = self.system_event_instances.0.write().unwrap();
        match event {
            SystemEvent::ComponentCreated(name) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::ComponentCreated).cloned() {
                    entity_instance.set("event", json!(name));
                }
            }
            SystemEvent::ComponentDeleted(name) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::ComponentDeleted).cloned() {
                    entity_instance.set("event", json!(name));
                }
            }
            SystemEvent::EntityTypeCreated(name) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::EntityTypeCreated).cloned() {
                    entity_instance.set("event", json!(name));
                }
            }
            SystemEvent::EntityTypeDeleted(name) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::EntityTypeDeleted).cloned() {
                    entity_instance.set("event", json!(name));
                }
            }
            SystemEvent::RelationTypeCreated(name) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::RelationTypeCreated).cloned() {
                    entity_instance.set("event", json!(name));
                }
            }
            SystemEvent::RelationTypeDeleted(name) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::RelationTypeDeleted).cloned() {
                    entity_instance.set("event", json!(name));
                }
            }
            SystemEvent::EntityInstanceCreated(id) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::EntityInstanceCreated).cloned() {
                    entity_instance.set("event", json!(id));
                }
            }
            SystemEvent::EntityInstanceDeleted(id) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::EntityInstanceDeleted).cloned() {
                    entity_instance.set("event", json!(id));
                }
            }
            SystemEvent::RelationInstanceCreated(edge_key) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::RelationInstanceCreated).cloned() {
                    entity_instance.set("event", json!(edge_key));
                }
            }
            SystemEvent::RelationInstanceDeleted(edge_key) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::RelationInstanceDeleted).cloned() {
                    entity_instance.set("event", json!(edge_key));
                }
            }
            SystemEvent::FlowCreated(id) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::FlowCreated).cloned() {
                    entity_instance.set("event", json!(id));
                }
            }
            SystemEvent::FlowDeleted(id) => {
                if let Some(entity_instance) = writer.get(&SystemEventTypes::FlowDeleted).cloned() {
                    entity_instance.set("event", json!(id));
                }
            }
        }
    }

    fn get_system_event_instances(&self) -> Vec<Arc<ReactiveEntityInstance>> {
        let reader = self.system_event_instances.0.read().unwrap();
        reader.values().cloned().collect()
    }
}

impl SystemEventManagerImpl {
    pub(crate) fn create_system_event_instances(&self) {
        let mut writer = self.system_event_instances.0.write().unwrap();
        writer.insert(
            SystemEventTypes::ComponentCreated,
            self.create_system_event_instance("/org/inexor/event/type/component/created"),
        );
        writer.insert(
            SystemEventTypes::ComponentDeleted,
            self.create_system_event_instance("/org/inexor/event/type/component/deleted"),
        );
        writer.insert(
            SystemEventTypes::EntityTypeCreated,
            self.create_system_event_instance("/org/inexor/event/type/entity/created"),
        );
        writer.insert(
            SystemEventTypes::EntityTypeDeleted,
            self.create_system_event_instance("/org/inexor/event/type/entity/deleted"),
        );
        writer.insert(
            SystemEventTypes::RelationTypeCreated,
            self.create_system_event_instance("/org/inexor/event/type/relation/created"),
        );
        writer.insert(
            SystemEventTypes::RelationTypeDeleted,
            self.create_system_event_instance("/org/inexor/event/type/relation/deleted"),
        );
        writer.insert(
            SystemEventTypes::EntityInstanceCreated,
            self.create_system_event_instance("/org/inexor/event/instance/entity/created"),
        );
        writer.insert(
            SystemEventTypes::EntityInstanceDeleted,
            self.create_system_event_instance("/org/inexor/event/instance/entity/deleted"),
        );
        writer.insert(
            SystemEventTypes::RelationInstanceCreated,
            self.create_system_event_instance("/org/inexor/event/instance/relation/created"),
        );
        writer.insert(
            SystemEventTypes::RelationInstanceDeleted,
            self.create_system_event_instance("/org/inexor/event/instance/relation/deleted"),
        );
        writer.insert(SystemEventTypes::FlowCreated, self.create_system_event_instance("/org/inexor/event/flow/created"));
        writer.insert(SystemEventTypes::FlowDeleted, self.create_system_event_instance("/org/inexor/event/flow/deleted"));
    }

    pub(crate) fn create_system_event_instance<S: Into<String>>(&self, label: S) -> Arc<ReactiveEntityInstance> {
        ReactiveEntityInstanceBuilder::new("system_event")
            .property("label", json!(label.into()))
            .property("event", json!(false))
            .get()
    }

    pub(crate) fn delete_system_event_instances(&self) {
        // let mut writer = self.event_instances.0.write().unwrap();
    }
}

impl Lifecycle for SystemEventManagerImpl {
    fn init(&self) {}

    fn post_init(&self) {
        self.create_system_event_instances();
    }

    fn pre_shutdown(&self) {
        self.delete_system_event_instances();
    }

    fn shutdown(&self) {}
}
