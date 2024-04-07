use async_trait::async_trait;
use serde_json::Value;
use springtime_di::injectable;
use uuid::Uuid;

use crate::ReactiveRelationComponentRemoveError;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_reactive_model_impl::ReactiveRelation;

use crate::ReactiveRelationComponentAddError;
use crate::ReactiveRelationCreationError;
use crate::ReactiveRelationPropertyAddError;
use crate::ReactiveRelationPropertyRemoveError;
use crate::ReactiveRelationRegistrationError;
use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_model_api::ComponentBehaviourTypeId;
use reactive_graph_behaviour_model_api::RelationBehaviourTypeId;
use reactive_graph_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait ReactiveRelationManager: Send + Sync + Lifecycle {
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

    /// Returns all reactive relation instances of the given type which are of the given component..
    fn get_by_component(&self, component_ty: &ComponentTypeId) -> Vec<ReactiveRelation>;

    /// Returns all reactive relation instances of the given type which behaves as the given behaviour.
    fn get_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> Vec<ReactiveRelation>;

    /// Returns all reactive relation instances of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> Vec<ReactiveRelation>;

    /// Returns all edge keys.
    fn get_keys(&self) -> Vec<RelationInstanceId>;

    /// Returns the count of registered reactive relation instances.
    fn count(&self) -> usize;

    /// Returns the count of registered reactive relation instances of the given type.
    fn count_by_type(&self, ty: &RelationTypeId) -> usize;

    /// Returns the count of registered reactive relation instances which are of the given component.
    fn count_by_component(&self, component_ty: &ComponentTypeId) -> usize;

    /// Returns the count of registered reactive relation instances which behaves as the given behaviour.
    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize;

    /// Creates a new reactive relation instance.
    fn create_reactive_relation(&self, id: &RelationInstanceId, properties: PropertyInstances) -> Result<ReactiveRelation, ReactiveRelationCreationError>;

    fn create_reactive_instance(&self, relation_instance: RelationInstance) -> Result<ReactiveRelation, ReactiveRelationCreationError>;

    /// Registers the given reactive relation instance and applies components and behaviours.
    fn register_reactive_instance(&self, relation_instance: ReactiveRelation) -> Result<ReactiveRelation, ReactiveRelationRegistrationError>;

    fn register_or_merge_reactive_instance(&self, relation_instance: ReactiveRelation) -> Result<ReactiveRelation, ReactiveRelationRegistrationError>;

    /// Adds the component with the given name to the relation instance with the given edge key.
    fn add_component(&self, id: &RelationInstanceId, component_ty: &ComponentTypeId) -> Result<(), ReactiveRelationComponentAddError>;

    /// Removes the component with the given name from the relation instance with the given edge key.
    fn remove_component(&self, id: &RelationInstanceId, component_ty: &ComponentTypeId) -> Result<(), ReactiveRelationComponentRemoveError>;

    /// Adds the property with the given name and initial value to the relation instance with the given id.
    fn add_property(&self, id: &RelationInstanceId, property_name: &str, mutability: Mutability, value: Value) -> Result<(), ReactiveRelationPropertyAddError>;

    /// Removes the property with the given name from the relation instance with the given id.
    fn remove_property(&self, id: &RelationInstanceId, property_name: &str) -> Result<(), ReactiveRelationPropertyRemoveError>;

    /// Adds the given behaviour to all instances of the given relation type.
    fn add_behaviour_to_all_relation_instances(&self, relation_behaviour_ty: &RelationBehaviourTypeId);

    /// Adds the given behaviour to all instances of the given relation type.
    fn add_behaviour_to_all_relation_components(&self, component_behaviour_ty: &ComponentBehaviourTypeId);

    // // TODO: fn commit(&self, relation_instance: RelationInstance);
    // // TODO: return result
    // fn commit(&self, id: &RelationInstanceId);

    /// Deletes the reactive relation instance with the given key.
    fn delete(&self, id: &RelationInstanceId) -> bool;

    fn unregister_reactive_instance(&self, id: &RelationInstanceId);

    fn handle_component_added_events(&self);

    fn handle_component_removed_events(&self);

    fn handle_property_added_events(&self);

    fn handle_property_removed_events(&self);
}
