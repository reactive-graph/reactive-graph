use crate::model::ComponentType;
use crate::model::EntityTypeType;
use crate::model::FlowTypeType;
use crate::model::RelationTypeType;
use indradb::EdgeKey;
use uuid::Uuid;

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
    ComponentCreated(ComponentType),
    // TODO: Replace ComponentUpdated with more concrete events
    ComponentUpdated(ComponentType),
    // TODO: ComponentPropertyAdded
    // TODO: ComponentPropertyRemoved
    // TODO: ComponentExtensionAdded
    // TODO: ComponentExtensionRemoved
    ComponentDeleted(ComponentType),
    EntityTypeCreated(EntityTypeType),
    EntityTypeComponentAdded(EntityTypeType, ComponentType),
    EntityTypeComponentRemoved(EntityTypeType, ComponentType),
    EntityTypePropertyAdded(EntityTypeType, String),
    EntityTypePropertyRemoved(EntityTypeType, String),
    EntityTypeExtensionAdded(EntityTypeType, String),
    EntityTypeExtensionRemoved(EntityTypeType, String),
    EntityTypeDeleted(EntityTypeType),
    RelationTypeCreated(RelationTypeType),
    RelationTypeComponentAdded(RelationTypeType, ComponentType),
    RelationTypeComponentRemoved(RelationTypeType, ComponentType),
    RelationTypePropertyAdded(RelationTypeType, String),
    RelationTypePropertyRemoved(RelationTypeType, String),
    RelationTypeExtensionAdded(RelationTypeType, String),
    RelationTypeExtensionRemoved(RelationTypeType, String),
    RelationTypeDeleted(RelationTypeType),
    FlowTypeCreated(FlowTypeType),
    FlowTypeUpdated(FlowTypeType),
    FlowTypeDeleted(FlowTypeType),
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
