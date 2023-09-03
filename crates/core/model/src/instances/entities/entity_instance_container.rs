use uuid::Uuid;

use crate::AddEntityInstanceError;
use crate::EntityInstance;
use crate::EntityInstances;
use crate::RemoveEntityInstanceError;
use crate::UpdateEntityInstanceError;

/// Container for entity instances.
pub trait EntityInstanceContainer {

    /// Returns the entity instances.
    fn entity_instances(&self) -> EntityInstances;

    // TODO:
    // fn get_entity_instance_ids(&self) -> DashSet<Uuid>;

    // TODO:
    // fn get_entity_type_ids(&self) -> EntityTypeIds;

    /// Returns true, if the container has an entity instance with the given id.
    fn has_entity_instance(&self, id: Uuid) -> bool;

    /// Adds the given entity instance.
    fn add_entity_instance(&self, entity_instance: EntityInstance) -> Result<(), AddEntityInstanceError>;

    /// Updates the entity instance with the id of the given entity instance.
    fn update_entity_instance(&self, id: Uuid, entity_instance: EntityInstance) -> Result<(Uuid, EntityInstance), UpdateEntityInstanceError>;

    /// Removes the entity instance with the given id.
    fn remove_entity_instance(&self, id: Uuid) -> Result<Option<(Uuid, EntityInstance)>, RemoveEntityInstanceError>;

    // TODO: Replaces the id of the entity instance and in all relation outbounds and relation inbounds.
    // fn set_entity_instance_id(&self, id: Uuid)
}


/// Collection of a type which contains entity instances.
// TODO: Rename T to TypeId
// TODO: Trait bound: T: NamespacedTypeGetter
// TODO: Trait bound: Self: NamespacedTypeContainer
pub trait NamespacedTypeEntityInstanceContainer<T, NamespacedTypeDoesNotExistError, AddEntityInstanceError, UpdateEntityInstanceError, RemoveEntityInstanceError> {
    /// Returns the entity instances.
    fn entity_instances(&self, ty: &T) -> Result<EntityInstances, NamespacedTypeDoesNotExistError>;

    // TODO:
    // fn get_entity_instance_ids(&self, ty: &T) -> Result<DashSet<Uuid>, NamespacedTypeDoesNotExistError>;

    // TODO:
    // fn get_entity_type_ids(&self, ty: &T) -> Result<EntityTypeIds, NamespacedTypeDoesNotExistError>;

    /// Returns true, if an entity instance with the given id exists.
    fn has_entity_instance(&self, ty: &T, id: Uuid) -> Result<bool, NamespacedTypeDoesNotExistError>;

    /// Adds the given entity instance.
    fn add_entity_instance(&self, ty: &T, entity_instance: EntityInstance) -> Result<(), AddEntityInstanceError>;

    /// Updates the entity instance with the id of the given entity instance.
    fn update_entity_instance(&self, ty: &T, id: Uuid, entity_instance: EntityInstance) -> Result<(Uuid, EntityInstance), UpdateEntityInstanceError>;

    /// Removes the entity instance with the given id.
    fn remove_entity_instance(&self, ty: &T, id: Uuid) -> Result<Option<(Uuid, EntityInstance)>, RemoveEntityInstanceError>;
}
