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
    ComponentUpdated,
    ComponentDeleted,
    EntityTypeCreated,
    EntityTypeUpdated,
    EntityTypeDeleted,
    RelationTypeCreated,
    RelationTypeUpdated,
    RelationTypeDeleted,
    FlowTypeCreated,
    FlowTypeUpdated,
    FlowTypeDeleted,

    /// The type system has changed
    TypeSystemChanged,

    EntityInstanceCreated,
    EntityInstanceDeleted,
    RelationInstanceCreated,
    RelationInstanceDeleted,
    FlowInstanceCreated,
    FlowInstanceDeleted,
}

pub enum SystemEvent {
    ComponentCreated(String),
    ComponentUpdated(String),
    ComponentDeleted(String),
    EntityTypeCreated(String),
    EntityTypeUpdated(String),
    EntityTypeDeleted(String),
    RelationTypeCreated(String),
    RelationTypeUpdated(String),
    RelationTypeDeleted(String),
    FlowTypeCreated(String),
    FlowTypeUpdated(String),
    FlowTypeDeleted(String),
    TypeSystemChanged,
    EntityInstanceCreated(Uuid),
    EntityInstanceDeleted(Uuid),
    RelationInstanceCreated(EdgeKey),
    RelationInstanceDeleted(EdgeKey),
    FlowInstanceCreated(Uuid),
    FlowInstanceDeleted(Uuid),
}

impl From<&SystemEvent> for SystemEventTypes {
    fn from(event: &SystemEvent) -> Self {
        match event {
            SystemEvent::ComponentCreated(_) => SystemEventTypes::ComponentCreated,
            SystemEvent::ComponentUpdated(_) => SystemEventTypes::ComponentUpdated,
            SystemEvent::ComponentDeleted(_) => SystemEventTypes::ComponentDeleted,
            SystemEvent::EntityTypeCreated(_) => SystemEventTypes::EntityTypeCreated,
            SystemEvent::EntityTypeUpdated(_) => SystemEventTypes::EntityTypeUpdated,
            SystemEvent::EntityTypeDeleted(_) => SystemEventTypes::EntityTypeDeleted,
            SystemEvent::RelationTypeCreated(_) => SystemEventTypes::RelationTypeCreated,
            SystemEvent::RelationTypeUpdated(_) => SystemEventTypes::RelationTypeUpdated,
            SystemEvent::RelationTypeDeleted(_) => SystemEventTypes::RelationTypeDeleted,
            SystemEvent::FlowTypeCreated(_) => SystemEventTypes::FlowTypeCreated,
            SystemEvent::FlowTypeUpdated(_) => SystemEventTypes::FlowTypeUpdated,
            SystemEvent::FlowTypeDeleted(_) => SystemEventTypes::FlowTypeDeleted,
            SystemEvent::TypeSystemChanged => SystemEventTypes::TypeSystemChanged,
            SystemEvent::EntityInstanceCreated(_) => SystemEventTypes::EntityInstanceCreated,
            SystemEvent::EntityInstanceDeleted(_) => SystemEventTypes::EntityInstanceDeleted,
            SystemEvent::RelationInstanceCreated(_) => SystemEventTypes::RelationInstanceCreated,
            SystemEvent::RelationInstanceDeleted(_) => SystemEventTypes::RelationInstanceDeleted,
            SystemEvent::FlowInstanceCreated(_) => SystemEventTypes::FlowInstanceCreated,
            SystemEvent::FlowInstanceDeleted(_) => SystemEventTypes::FlowInstanceDeleted,
        }
    }
}

#[async_trait]
pub trait SystemEventManager: Send + Sync + Lifecycle {
    fn emit_event(&self, event: SystemEvent);

    fn get_system_event_instances(&self) -> Vec<Arc<ReactiveEntityInstance>>;

    fn get_system_event_instance(&self, event_type: SystemEventTypes) -> Option<Arc<ReactiveEntityInstance>>;
}
