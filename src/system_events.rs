use indradb::EdgeKey;
use uuid::Uuid;

use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::FlowTypeId;
use crate::model::RelationTypeId;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum SystemEventTypes {
    ComponentCreated,
    ComponentUpdated,
    ComponentDeleted,
    EntityTypeCreated,
    EntityTypeComponentAdded,
    EntityTypeComponentRemoved,
    EntityTypePropertyAdded,
    EntityTypePropertyRemoved,
    EntityTypeExtensionAdded,
    EntityTypeExtensionRemoved,
    EntityTypeDeleted,
    RelationTypeCreated,
    RelationTypeComponentAdded,
    RelationTypeComponentRemoved,
    RelationTypePropertyAdded,
    RelationTypePropertyRemoved,
    RelationTypeExtensionAdded,
    RelationTypeExtensionRemoved,
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
    ComponentCreated(ComponentTypeId),
    // TODO: Replace ComponentUpdated with more concrete events
    ComponentUpdated(ComponentTypeId),
    // TODO: ComponentPropertyAdded
    // TODO: ComponentPropertyRemoved
    // TODO: ComponentExtensionAdded
    // TODO: ComponentExtensionRemoved
    ComponentDeleted(ComponentTypeId),
    EntityTypeCreated(EntityTypeId),
    EntityTypeComponentAdded(EntityTypeId, ComponentTypeId),
    EntityTypeComponentRemoved(EntityTypeId, ComponentTypeId),
    EntityTypePropertyAdded(EntityTypeId, String),
    EntityTypePropertyRemoved(EntityTypeId, String),
    EntityTypeExtensionAdded(EntityTypeId, String),
    EntityTypeExtensionRemoved(EntityTypeId, String),
    EntityTypeDeleted(EntityTypeId),
    RelationTypeCreated(RelationTypeId),
    RelationTypeComponentAdded(RelationTypeId, ComponentTypeId),
    RelationTypeComponentRemoved(RelationTypeId, ComponentTypeId),
    RelationTypePropertyAdded(RelationTypeId, String),
    RelationTypePropertyRemoved(RelationTypeId, String),
    RelationTypeExtensionAdded(RelationTypeId, String),
    RelationTypeExtensionRemoved(RelationTypeId, String),
    RelationTypeDeleted(RelationTypeId),
    FlowTypeCreated(FlowTypeId),
    FlowTypeUpdated(FlowTypeId),
    FlowTypeDeleted(FlowTypeId),
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
            SystemEvent::EntityTypeComponentAdded(_, _) => SystemEventTypes::EntityTypeComponentAdded,
            SystemEvent::EntityTypeComponentRemoved(_, _) => SystemEventTypes::EntityTypeComponentRemoved,
            SystemEvent::EntityTypePropertyAdded(_, _) => SystemEventTypes::EntityTypePropertyAdded,
            SystemEvent::EntityTypePropertyRemoved(_, _) => SystemEventTypes::EntityTypePropertyRemoved,
            SystemEvent::EntityTypeExtensionAdded(_, _) => SystemEventTypes::EntityTypeExtensionAdded,
            SystemEvent::EntityTypeExtensionRemoved(_, _) => SystemEventTypes::EntityTypeExtensionRemoved,
            SystemEvent::EntityTypeDeleted(_) => SystemEventTypes::EntityTypeDeleted,
            SystemEvent::RelationTypeCreated(_) => SystemEventTypes::RelationTypeCreated,
            SystemEvent::RelationTypeComponentAdded(_, _) => SystemEventTypes::RelationTypeComponentAdded,
            SystemEvent::RelationTypeComponentRemoved(_, _) => SystemEventTypes::RelationTypeComponentRemoved,
            SystemEvent::RelationTypePropertyAdded(_, _) => SystemEventTypes::RelationTypePropertyAdded,
            SystemEvent::RelationTypePropertyRemoved(_, _) => SystemEventTypes::RelationTypePropertyRemoved,
            SystemEvent::RelationTypeExtensionAdded(_, _) => SystemEventTypes::RelationTypeExtensionAdded,
            SystemEvent::RelationTypeExtensionRemoved(_, _) => SystemEventTypes::RelationTypeExtensionRemoved,
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
