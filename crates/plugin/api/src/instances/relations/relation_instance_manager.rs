use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_behaviour_model_api::BehaviourTypeId;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::Mutability;
use inexor_rgf_graph::RelationInstance;
use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_reactive_model_impl::ReactiveRelation;
use inexor_rgf_reactive_service_api::ReactiveRelationComponentAddError;
use inexor_rgf_reactive_service_api::ReactiveRelationComponentRemoveError;
use inexor_rgf_reactive_service_api::ReactiveRelationCreationError;
use inexor_rgf_reactive_service_api::ReactiveRelationPropertyAddError;
use inexor_rgf_reactive_service_api::ReactiveRelationPropertyRemoveError;
use inexor_rgf_reactive_service_api::ReactiveRelationRegistrationError;

#[derive(Debug)]
pub enum RelationInstanceManagerError {
    InitializationError,
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
    fn create(&self, relation_instance: RelationInstance) -> Result<ReactiveRelation, ReactiveRelationCreationError>;

    /// Registers the given reactive relation instance and applies components and behaviours.
    fn register(&self, reactive_relation: ReactiveRelation) -> Result<ReactiveRelation, ReactiveRelationRegistrationError>;

    /// Adds the component with the given name to the relation instance with the given edge key.
    fn add_component(&self, edge_key: &RelationInstanceId, component: &ComponentTypeId) -> Result<(), ReactiveRelationComponentAddError>;

    /// Removes the component with the given name from the relation instance with the given edge key.
    fn remove_component(&self, edge_key: &RelationInstanceId, component: &ComponentTypeId) -> Result<(), ReactiveRelationComponentRemoveError>;

    /// Adds the property with the given name and initial value to the relation instance with the given id.
    fn add_property(&self, id: &RelationInstanceId, property_name: &str, mutability: Mutability, value: Value) -> Result<(), ReactiveRelationPropertyAddError>;

    /// Removes the property with the given name from the relation instance with the given id.
    fn remove_property(&self, id: &RelationInstanceId, property_name: &str) -> Result<(), ReactiveRelationPropertyRemoveError>;

    /// Deletes the reactive relation instance with the given key.
    fn delete(&self, edge_key: &RelationInstanceId) -> bool;
}
