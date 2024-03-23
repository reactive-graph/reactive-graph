use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_graph::ExtensionTypeId;
use inexor_rgf_graph::FlowTypeId;
use inexor_rgf_graph::RelationTypeId;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum TypeSystemEventTypes {
    ComponentCreated,
    ComponentPropertyAdded,
    ComponentPropertyRenamed,
    ComponentPropertyUpdated,
    ComponentPropertyRemoved,
    ComponentExtensionAdded,
    ComponentExtensionRenamed,
    ComponentExtensionUpdated,
    ComponentExtensionRemoved,
    ComponentDeleted,
    EntityTypeCreated,
    EntityTypeComponentAdded,
    EntityTypeComponentRenamed,
    EntityTypeComponentUpdated,
    EntityTypeComponentRemoved,
    EntityTypePropertyAdded,
    EntityTypePropertyRenamed,
    EntityTypePropertyUpdated,
    EntityTypePropertyRemoved,
    EntityTypeExtensionAdded,
    EntityTypeExtensionRenamed,
    EntityTypeExtensionUpdated,
    EntityTypeExtensionRemoved,
    EntityTypeDeleted,
    RelationTypeCreated,
    RelationTypeComponentAdded,
    RelationTypeComponentRenamed,
    RelationTypeComponentUpdated,
    RelationTypeComponentRemoved,
    RelationTypePropertyAdded,
    RelationTypePropertyRenamed,
    RelationTypePropertyUpdated,
    RelationTypePropertyRemoved,
    RelationTypeExtensionAdded,
    RelationTypeExtensionRenamed,
    RelationTypeExtensionUpdated,
    RelationTypeExtensionRemoved,
    RelationTypeDeleted,
    FlowTypeCreated,
    FlowTypeUpdated,
    FlowTypeDeleted,

    /// The type system has changed
    TypeSystemChanged,
}

pub enum TypeSystemEvent {
    ComponentCreated(ComponentTypeId),
    ComponentPropertyAdded(ComponentTypeId, String),
    ComponentPropertyRenamed(ComponentTypeId, String, String),
    ComponentPropertyUpdated(ComponentTypeId, String),
    ComponentPropertyRemoved(ComponentTypeId, String),
    ComponentExtensionAdded(ComponentTypeId, ExtensionTypeId),
    ComponentExtensionRenamed(ComponentTypeId, ExtensionTypeId, ExtensionTypeId),
    ComponentExtensionUpdated(ComponentTypeId, ExtensionTypeId),
    ComponentExtensionRemoved(ComponentTypeId, ExtensionTypeId),
    ComponentDeleted(ComponentTypeId),
    EntityTypeCreated(EntityTypeId),
    EntityTypeComponentAdded(EntityTypeId, ComponentTypeId),
    EntityTypeComponentRenamed(EntityTypeId, ComponentTypeId, ComponentTypeId),
    EntityTypeComponentUpdated(EntityTypeId, ComponentTypeId),
    EntityTypeComponentRemoved(EntityTypeId, ComponentTypeId),
    EntityTypePropertyAdded(EntityTypeId, String),
    EntityTypePropertyRenamed(EntityTypeId, String, String),
    EntityTypePropertyUpdated(EntityTypeId, String),
    EntityTypePropertyRemoved(EntityTypeId, String),
    EntityTypeExtensionAdded(EntityTypeId, ExtensionTypeId),
    EntityTypeExtensionRenamed(EntityTypeId, ExtensionTypeId, ExtensionTypeId),
    EntityTypeExtensionUpdated(EntityTypeId, ExtensionTypeId),
    EntityTypeExtensionRemoved(EntityTypeId, ExtensionTypeId),
    EntityTypeDeleted(EntityTypeId),
    RelationTypeCreated(RelationTypeId),
    RelationTypeComponentAdded(RelationTypeId, ComponentTypeId),
    RelationTypeComponentRenamed(RelationTypeId, ComponentTypeId, ComponentTypeId),
    RelationTypeComponentUpdated(RelationTypeId, ComponentTypeId),
    RelationTypeComponentRemoved(RelationTypeId, ComponentTypeId),
    RelationTypePropertyAdded(RelationTypeId, String),
    RelationTypePropertyRenamed(RelationTypeId, String, String),
    RelationTypePropertyUpdated(RelationTypeId, String),
    RelationTypePropertyRemoved(RelationTypeId, String),
    RelationTypeExtensionAdded(RelationTypeId, ExtensionTypeId),
    RelationTypeExtensionRenamed(RelationTypeId, ExtensionTypeId, ExtensionTypeId),
    RelationTypeExtensionUpdated(RelationTypeId, ExtensionTypeId),
    RelationTypeExtensionRemoved(RelationTypeId, ExtensionTypeId),
    RelationTypeDeleted(RelationTypeId),
    FlowTypeCreated(FlowTypeId),
    // TODO: Replace FlowTypeUpdated with more concrete events
    FlowTypeUpdated(FlowTypeId),
    FlowTypeDeleted(FlowTypeId),
    TypeSystemChanged,
}

impl From<&TypeSystemEvent> for TypeSystemEventTypes {
    fn from(event: &TypeSystemEvent) -> Self {
        match event {
            TypeSystemEvent::ComponentCreated(_) => TypeSystemEventTypes::ComponentCreated,
            TypeSystemEvent::ComponentPropertyAdded(_, _) => TypeSystemEventTypes::ComponentPropertyAdded,
            TypeSystemEvent::ComponentPropertyRenamed(_, _, _) => TypeSystemEventTypes::ComponentPropertyRenamed,
            TypeSystemEvent::ComponentPropertyUpdated(_, _) => TypeSystemEventTypes::ComponentPropertyUpdated,
            TypeSystemEvent::ComponentPropertyRemoved(_, _) => TypeSystemEventTypes::ComponentPropertyRemoved,
            TypeSystemEvent::ComponentExtensionAdded(_, _) => TypeSystemEventTypes::ComponentExtensionAdded,
            TypeSystemEvent::ComponentExtensionRenamed(_, _, _) => TypeSystemEventTypes::ComponentExtensionRenamed,
            TypeSystemEvent::ComponentExtensionUpdated(_, _) => TypeSystemEventTypes::ComponentExtensionUpdated,
            TypeSystemEvent::ComponentExtensionRemoved(_, _) => TypeSystemEventTypes::ComponentExtensionRemoved,
            TypeSystemEvent::ComponentDeleted(_) => TypeSystemEventTypes::ComponentDeleted,
            TypeSystemEvent::EntityTypeCreated(_) => TypeSystemEventTypes::EntityTypeCreated,
            TypeSystemEvent::EntityTypeComponentAdded(_, _) => TypeSystemEventTypes::EntityTypeComponentAdded,
            TypeSystemEvent::EntityTypeComponentRenamed(_, _, _) => TypeSystemEventTypes::EntityTypeComponentRenamed,
            TypeSystemEvent::EntityTypeComponentUpdated(_, _) => TypeSystemEventTypes::EntityTypeComponentUpdated,
            TypeSystemEvent::EntityTypeComponentRemoved(_, _) => TypeSystemEventTypes::EntityTypeComponentRemoved,
            TypeSystemEvent::EntityTypePropertyAdded(_, _) => TypeSystemEventTypes::EntityTypePropertyAdded,
            TypeSystemEvent::EntityTypePropertyRenamed(_, _, _) => TypeSystemEventTypes::EntityTypePropertyRenamed,
            TypeSystemEvent::EntityTypePropertyUpdated(_, _) => TypeSystemEventTypes::EntityTypePropertyUpdated,
            TypeSystemEvent::EntityTypePropertyRemoved(_, _) => TypeSystemEventTypes::EntityTypePropertyRemoved,
            TypeSystemEvent::EntityTypeExtensionAdded(_, _) => TypeSystemEventTypes::EntityTypeExtensionAdded,
            TypeSystemEvent::EntityTypeExtensionRenamed(_, _, _) => TypeSystemEventTypes::EntityTypeExtensionRenamed,
            TypeSystemEvent::EntityTypeExtensionUpdated(_, _) => TypeSystemEventTypes::EntityTypeExtensionUpdated,
            TypeSystemEvent::EntityTypeExtensionRemoved(_, _) => TypeSystemEventTypes::EntityTypeExtensionRemoved,
            TypeSystemEvent::EntityTypeDeleted(_) => TypeSystemEventTypes::EntityTypeDeleted,
            TypeSystemEvent::RelationTypeCreated(_) => TypeSystemEventTypes::RelationTypeCreated,
            TypeSystemEvent::RelationTypeComponentAdded(_, _) => TypeSystemEventTypes::RelationTypeComponentAdded,
            TypeSystemEvent::RelationTypeComponentRenamed(_, _, _) => TypeSystemEventTypes::RelationTypeComponentRenamed,
            TypeSystemEvent::RelationTypeComponentUpdated(_, _) => TypeSystemEventTypes::RelationTypeComponentUpdated,
            TypeSystemEvent::RelationTypeComponentRemoved(_, _) => TypeSystemEventTypes::RelationTypeComponentRemoved,
            TypeSystemEvent::RelationTypePropertyAdded(_, _) => TypeSystemEventTypes::RelationTypePropertyAdded,
            TypeSystemEvent::RelationTypePropertyRenamed(_, _, _) => TypeSystemEventTypes::RelationTypePropertyRenamed,
            TypeSystemEvent::RelationTypePropertyUpdated(_, _) => TypeSystemEventTypes::RelationTypePropertyUpdated,
            TypeSystemEvent::RelationTypePropertyRemoved(_, _) => TypeSystemEventTypes::RelationTypePropertyRemoved,
            TypeSystemEvent::RelationTypeExtensionAdded(_, _) => TypeSystemEventTypes::RelationTypeExtensionAdded,
            TypeSystemEvent::RelationTypeExtensionRenamed(_, _, _) => TypeSystemEventTypes::RelationTypeExtensionRenamed,
            TypeSystemEvent::RelationTypeExtensionUpdated(_, _) => TypeSystemEventTypes::RelationTypeExtensionUpdated,
            TypeSystemEvent::RelationTypeExtensionRemoved(_, _) => TypeSystemEventTypes::RelationTypeExtensionRemoved,
            TypeSystemEvent::RelationTypeDeleted(_) => TypeSystemEventTypes::RelationTypeDeleted,
            TypeSystemEvent::FlowTypeCreated(_) => TypeSystemEventTypes::FlowTypeCreated,
            TypeSystemEvent::FlowTypeUpdated(_) => TypeSystemEventTypes::FlowTypeUpdated,
            TypeSystemEvent::FlowTypeDeleted(_) => TypeSystemEventTypes::FlowTypeDeleted,
            TypeSystemEvent::TypeSystemChanged => TypeSystemEventTypes::TypeSystemChanged,
        }
    }
}