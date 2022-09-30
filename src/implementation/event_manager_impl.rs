use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use serde_json::json;

use crate::api::Lifecycle;
use crate::api::SystemEvent;
use crate::api::SystemEventManager;
use crate::api::SystemEventTypes;
use crate::api::SYSTEM_EVENT_PROPERTY_EVENT;
use crate::api::SYSTEM_EVENT_PROPERTY_LABEL;
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
        if let Some(entity_instance) = self.get_system_event_instance((&event).into()) {
            match event {
                SystemEvent::ComponentCreated(name)
                | SystemEvent::ComponentUpdated(name)
                | SystemEvent::ComponentDeleted(name)
                | SystemEvent::EntityTypeCreated(name)
                | SystemEvent::EntityTypeUpdated(name)
                | SystemEvent::EntityTypeDeleted(name)
                | SystemEvent::RelationTypeCreated(name)
                | SystemEvent::RelationTypeUpdated(name)
                | SystemEvent::RelationTypeDeleted(name)
                | SystemEvent::FlowTypeCreated(name)
                | SystemEvent::FlowTypeUpdated(name)
                | SystemEvent::FlowTypeDeleted(name) => {
                    entity_instance.set(SYSTEM_EVENT_PROPERTY_EVENT, json!(name));
                    // Also emit event that the type system has been changed
                    self.emit_event(SystemEvent::TypeSystemChanged);
                }
                SystemEvent::TypeSystemChanged => entity_instance.set(SYSTEM_EVENT_PROPERTY_EVENT, json!(true)),
                SystemEvent::EntityInstanceCreated(id)
                | SystemEvent::EntityInstanceDeleted(id)
                | SystemEvent::FlowInstanceCreated(id)
                | SystemEvent::FlowInstanceDeleted(id) => entity_instance.set(SYSTEM_EVENT_PROPERTY_EVENT, json!(id)),
                SystemEvent::RelationInstanceCreated(edge_key) | SystemEvent::RelationInstanceDeleted(edge_key) => {
                    entity_instance.set("event", json!(edge_key))
                }
            }
        }
    }

    fn get_system_event_instances(&self) -> Vec<Arc<ReactiveEntityInstance>> {
        let reader = self.system_event_instances.0.read().unwrap();
        reader.values().cloned().collect()
    }

    fn get_system_event_instance(&self, event_type: SystemEventTypes) -> Option<Arc<ReactiveEntityInstance>> {
        let reader = self.system_event_instances.0.read().unwrap();
        reader.get(&event_type).cloned()
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
            SystemEventTypes::ComponentUpdated,
            self.create_system_event_instance("/org/inexor/event/type/component/updated"),
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
            SystemEventTypes::EntityTypeUpdated,
            self.create_system_event_instance("/org/inexor/event/type/entity/updated"),
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
            SystemEventTypes::RelationTypeUpdated,
            self.create_system_event_instance("/org/inexor/event/type/relation/updated"),
        );
        writer.insert(
            SystemEventTypes::RelationTypeDeleted,
            self.create_system_event_instance("/org/inexor/event/type/relation/deleted"),
        );
        writer.insert(SystemEventTypes::FlowTypeCreated, self.create_system_event_instance("/org/inexor/event/type/flow/created"));
        writer.insert(SystemEventTypes::FlowTypeUpdated, self.create_system_event_instance("/org/inexor/event/type/flow/updated"));
        writer.insert(SystemEventTypes::FlowTypeDeleted, self.create_system_event_instance("/org/inexor/event/type/flow/deleted"));
        writer.insert(SystemEventTypes::TypeSystemChanged, self.create_system_event_instance("/org/inexor/event/type/changed"));
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
        writer.insert(
            SystemEventTypes::FlowInstanceCreated,
            self.create_system_event_instance("/org/inexor/event/instance/flow/created"),
        );
        writer.insert(
            SystemEventTypes::FlowInstanceDeleted,
            self.create_system_event_instance("/org/inexor/event/instance/flow/deleted"),
        );
    }

    pub(crate) fn create_system_event_instance<S: Into<String>>(&self, label: S) -> Arc<ReactiveEntityInstance> {
        ReactiveEntityInstanceBuilder::new("system_event")
            .property(SYSTEM_EVENT_PROPERTY_LABEL, json!(label.into()))
            .property(SYSTEM_EVENT_PROPERTY_EVENT, json!(false))
            .build()
    }

    pub(crate) fn delete_system_event_instances(&self) {
        // let mut writer = self.event_instances.0.write().unwrap();
    }
}

impl Lifecycle for SystemEventManagerImpl {
    fn post_init(&self) {
        self.create_system_event_instances();
    }

    fn pre_shutdown(&self) {
        self.delete_system_event_instances();
    }
}
