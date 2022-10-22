use std::sync::Arc;

use indradb::EdgeKey;
use uuid::Uuid;

use crate::model::ComponentType;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationInstance;
use crate::model::RelationTypeType;

#[derive(Debug)]
pub enum RelationInstanceManagerError {
    InitializationError,
}

#[derive(Debug)]
pub enum RelationInstanceCreationError {
    Failed,
}

#[derive(Debug)]
pub enum ReactiveRelationInstanceComponentAddError {
    Failed,
}

pub trait RelationInstanceManager: Send + Sync {
    /// Returns true, if an relation of the given type exists which starts at the given outbound entity and
    /// ends at the given inbound entity.
    fn has(&self, edge_key: &EdgeKey) -> bool;

    /// Returns the ReactiveRelationInstance with the given type_name, which starts at the given
    /// outbound entity and ends at the given inbound entity.
    fn get(&self, edge_key: &EdgeKey) -> Option<Arc<ReactiveRelationInstance>>;

    /// Returns all reactive relation instances of the given outbound entity instance.
    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all reactive relation instances of the given inbound entity instance.
    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all reactive relation instances.
    fn get_all(&self) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all reactive relation instances of the given type.
    fn get_by_type(&self, ty: &RelationTypeType) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all reactive relation instances of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all edge keys.
    fn get_keys(&self) -> Vec<EdgeKey>;

    /// Returns the count of registered reactive relation instances.
    fn count(&self) -> usize;

    /// Returns the count of registered reactive relation instances of the given type.
    fn count_by_type(&self, ty: &RelationTypeType) -> usize;

    /// Returns the count of registered reactive relation instances which are of the given component.
    fn count_by_component(&self, component: &ComponentType) -> usize;

    /// Returns the count of registered reactive relation instances which behaves as the given behaviour.
    fn count_by_behaviour(&self, behaviour: &str) -> usize;

    /// Creates a new reactive relation instance.
    fn create(&self, relation_instance: RelationInstance) -> Result<Arc<ReactiveRelationInstance>, RelationInstanceCreationError>;

    /// Adds the component with the given name to the relation instance with the given edge key.
    fn add_component(&self, edge_key: &EdgeKey, component: &ComponentType) -> Result<(), ReactiveRelationInstanceComponentAddError>;

    /// Removes the component with the given name from the relation instance with the given edge key.
    fn remove_component(&self, edge_key: &EdgeKey, component: &ComponentType);

    /// Deletes the reactive relation instance with the given key.
    fn delete(&self, edge_key: &EdgeKey) -> bool;
}
