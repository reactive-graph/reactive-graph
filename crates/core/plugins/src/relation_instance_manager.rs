use std::fmt;

use uuid::Uuid;
use crate::model::RelationInstanceId;

use crate::reactive::BehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::reactive::ReactiveRelation;
use crate::model::RelationInstance;
use crate::model::RelationTypeId;

#[derive(Debug)]
pub enum RelationInstanceManagerError {
    InitializationError,
}

#[derive(Debug)]
pub enum RelationInstanceCreationError {
    Failed,
}

impl fmt::Display for RelationInstanceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            RelationInstanceCreationError::Failed => {
                write!(f, "Failed to create relation instance")
            }
        }
    }
}

#[derive(Debug)]
pub enum ReactiveRelationComponentAddError {
    Failed,
}

pub trait RelationInstanceManager: Send + Sync {
    /// Returns true, if an relation of the given type exists which starts at the given outbound entity and
    /// ends at the given inbound entity.
    fn has(&self, id: &RelationInstanceId) -> bool;

    /// Returns the ReactiveRelation with the given type_name, which starts at the given
    /// outbound entity and ends at the given inbound entity.
    fn get(&self, id: &RelationInstanceId) -> Option<ReactiveRelation>;

    /// Returns all reactive relation instances of the given outbound entity instance.
    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<ReactiveRelation>;

    /// Returns all reactive relation instances of the given inbound entity instance.
    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<ReactiveRelation>;

    /// Returns all reactive relation instances.
    fn get_all(&self) -> Vec<ReactiveRelation>;

    /// Returns all reactive relation instances of the given type.
    fn get_by_type(&self, ty: &RelationTypeId) -> Vec<ReactiveRelation>;

    /// Returns all reactive relation instances of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> Vec<ReactiveRelation>;

    /// Returns all edge keys.
    fn get_keys(&self) -> Vec<RelationInstanceId>;

    /// Returns the count of registered reactive relation instances.
    fn count(&self) -> usize;

    /// Returns the count of registered reactive relation instances of the given type.
    fn count_by_type(&self, ty: &RelationTypeId) -> usize;

    /// Returns the count of registered reactive relation instances which are of the given component.
    fn count_by_component(&self, component: &ComponentTypeId) -> usize;

    /// Returns the count of registered reactive relation instances which behaves as the given behaviour.
    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize;

    /// Creates a new reactive relation instance.
    fn create(&self, relation_instance: RelationInstance) -> Result<ReactiveRelation, RelationInstanceCreationError>;

    /// Adds the component with the given name to the relation instance with the given edge key.
    fn add_component(&self, edge_key: &RelationInstanceId, component: &ComponentTypeId) -> Result<(), ReactiveRelationComponentAddError>;

    /// Removes the component with the given name from the relation instance with the given edge key.
    fn remove_component(&self, edge_key: &RelationInstanceId, component: &ComponentTypeId);

    /// Deletes the reactive relation instance with the given key.
    fn delete(&self, edge_key: &RelationInstanceId) -> bool;
}
