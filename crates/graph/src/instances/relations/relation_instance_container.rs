use uuid::Uuid;

use crate::AddRelationInstanceError;
use crate::RelationInstance;
use crate::RelationInstanceId;
use crate::RelationInstances;
use crate::RemoveRelationInstanceError;
use crate::UpdateRelationInstanceError;

/// Container for relation instances.
pub trait RelationInstanceContainer {
    /// Returns the relation instances.
    fn relation_instances(&self) -> RelationInstances;

    /// Returns true, if a relation instance exists which uses an entity instance with the given id.
    fn has_relation_which_uses_entity_instance(&self, id: Uuid) -> bool;

    /// Returns true, if a relation instance with the given id exists.
    fn has_relation_instance(&self, id: &RelationInstanceId) -> bool;

    /// Adds the given relation instance.
    fn add_relation_instance(&self, relation_instance: RelationInstance) -> Result<(), AddRelationInstanceError>;

    /// Updates the relation instance with the given id with the given relation instance.
    fn update_relation_instance(
        &self,
        id: &RelationInstanceId,
        relation_instance: RelationInstance,
    ) -> Result<(RelationInstanceId, RelationInstance), UpdateRelationInstanceError>;

    /// Removes the relation instance with the given id.
    fn remove_relation_instance(&self, id: &RelationInstanceId) -> Result<Option<(RelationInstanceId, RelationInstance)>, RemoveRelationInstanceError>;
}

/// Collection of a type which contains relation instances.
// TODO: Rename T to TypeId
// TODO: Trait bound: T: NamespacedTypeGetter
// TODO: Trait bound: Self: NamespacedTypeContainer
pub trait NamespacedTypeRelationInstanceContainer<
    T,
    NamespacedTypeDoesNotExistError,
    AddRelationInstanceError,
    UpdateRelationInstanceError,
    RemoveRelationInstanceError,
>
{
    /// Returns the relation instances.
    fn relation_instances(&self, ty: &T) -> Result<RelationInstances, NamespacedTypeDoesNotExistError>;

    /// Returns true, if a relation instance exists which uses an entity instance with the given id.
    fn has_relation_which_uses_entity_instance(&self, ty: &T, id: Uuid) -> Result<bool, NamespacedTypeDoesNotExistError>;

    /// Returns true, if a relation instance with the given id exists.
    fn has_relation_instance(&self, ty: &T, id: &RelationInstanceId) -> Result<bool, NamespacedTypeDoesNotExistError>;

    /// Adds the given relation instance.
    fn add_relation_instance(&self, ty: &T, relation_instance: RelationInstance) -> Result<(), AddRelationInstanceError>;

    /// Updates the relation instance with the id of the given relation instance.
    fn update_relation_instance(
        &self,
        ty: &T,
        id: &RelationInstanceId,
        relation_instance: RelationInstance,
    ) -> Result<(RelationInstanceId, RelationInstance), UpdateRelationInstanceError>;

    /// Removes the relation instance with the given id.
    fn remove_relation_instance(&self, ty: &T, id: &RelationInstanceId) -> Result<Option<(RelationInstanceId, RelationInstance)>, RemoveRelationInstanceError>;
}
