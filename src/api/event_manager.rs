use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::model::ReactiveEntityInstance;

pub const SYSTEM_EVENT_PROPERTY_EVENT: &str = "event";
pub const SYSTEM_EVENT_PROPERTY_LABEL: &str = "label";

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum SystemEventTypes {
    ComponentCreated,
    ComponentDeleted,
    EntityTypeCreated,
    EntityTypeDeleted,
    RelationTypeCreated,
    RelationTypeDeleted,

    /// The type system has changed
    TypeSystemChanged,

    EntityInstanceCreated,
    EntityInstanceDeleted,
    RelationInstanceCreated,
    RelationInstanceDeleted,
    FlowCreated,
    FlowDeleted,
}

pub enum SystemEvent {
    ComponentCreated(String),
    ComponentDeleted(String),
    EntityTypeCreated(String),
    EntityTypeDeleted(String),
    RelationTypeCreated(String),
    RelationTypeDeleted(String),
    TypeSystemChanged,
    EntityInstanceCreated(Uuid),
    EntityInstanceDeleted(Uuid),
    RelationInstanceCreated(EdgeKey),
    RelationInstanceDeleted(EdgeKey),
    FlowCreated(Uuid),
    FlowDeleted(Uuid),
}

impl From<&SystemEvent> for SystemEventTypes {
    fn from(event: &SystemEvent) -> Self {
        match event {
            SystemEvent::ComponentCreated(_) => SystemEventTypes::ComponentCreated,
            SystemEvent::ComponentDeleted(_) => SystemEventTypes::ComponentDeleted,
            SystemEvent::EntityTypeCreated(_) => SystemEventTypes::EntityTypeCreated,
            SystemEvent::EntityTypeDeleted(_) => SystemEventTypes::EntityTypeDeleted,
            SystemEvent::RelationTypeCreated(_) => SystemEventTypes::RelationTypeCreated,
            SystemEvent::RelationTypeDeleted(_) => SystemEventTypes::RelationTypeDeleted,
            SystemEvent::TypeSystemChanged => SystemEventTypes::TypeSystemChanged,
            SystemEvent::EntityInstanceCreated(_) => SystemEventTypes::EntityInstanceCreated,
            SystemEvent::EntityInstanceDeleted(_) => SystemEventTypes::EntityInstanceDeleted,
            SystemEvent::RelationInstanceCreated(_) => SystemEventTypes::RelationInstanceCreated,
            SystemEvent::RelationInstanceDeleted(_) => SystemEventTypes::RelationInstanceDeleted,
            SystemEvent::FlowCreated(_) => SystemEventTypes::FlowCreated,
            SystemEvent::FlowDeleted(_) => SystemEventTypes::FlowDeleted,
        }
    }
}

#[async_trait]
pub trait SystemEventManager: Send + Sync + Lifecycle {
    fn emit_event(&self, event: SystemEvent);

    fn get_system_event_instances(&self) -> Vec<Arc<ReactiveEntityInstance>>;

    fn get_system_event_instance(&self, event_type: SystemEventTypes) -> Option<Arc<ReactiveEntityInstance>>;
}
