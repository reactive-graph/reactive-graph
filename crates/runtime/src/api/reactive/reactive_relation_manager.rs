use std::fmt;

use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

// TODO: this is wrong?
use inexor_rgf_core_plugins::RelationInstanceCreationError;

use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::Mutability;
use crate::model::PropertyInstances;
use crate::model::RelationInstance;
use crate::model::RelationInstanceId;
use crate::model::RelationTypeId;
use crate::model::TypeDefinitionGetter;
use crate::reactive::BehaviourTypeId;
use crate::reactive::ComponentBehaviourTypeId;
use crate::reactive::ReactiveRelation;
use crate::reactive::RelationBehaviourTypeId;

#[derive(Debug)]
pub enum ReactiveRelationCreationError {
    InvalidEdgeKey,
    OutboundEntityDoesNotHaveComponent(RelationTypeId, ComponentTypeId),
    OutboundEntityIsNotOfType(RelationTypeId, EntityTypeId),
    InboundEntityDoesNotHaveComponent(RelationTypeId, ComponentTypeId),
    InboundEntityIsNotOfType(RelationTypeId, EntityTypeId),
    MissingOutboundEntityInstance(Uuid),
    MissingInboundEntityInstance(Uuid),
    /// No reactive relation instance with the given edge key exists.
    MissingInstance(RelationInstanceId),
    RelationInstanceCreationError(RelationInstanceCreationError),
    UnknownRelationType(RelationTypeId),
    // ValidationError(ValidationError),
    ReactiveRelationRegistrationError(ReactiveRelationRegistrationError),
}

impl fmt::Display for ReactiveRelationCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            // ReactiveRelationCreationError::UuidTaken(id) => write!(f, "The UUID {} has been already taken!", id),
            ReactiveRelationCreationError::InvalidEdgeKey => {
                write!(f, "The edge key is invalid")
            }
            ReactiveRelationCreationError::OutboundEntityDoesNotHaveComponent(relation_ty, component_ty) => {
                write!(f, "Relation type {} outbound relation instance doesn't have component {}", relation_ty, component_ty)
            }
            ReactiveRelationCreationError::OutboundEntityIsNotOfType(expected_ty, actual_ty) => {
                write!(
                    f,
                    "Unknown relation type: {} (expected: {})",
                    expected_ty.type_definition().to_string(),
                    actual_ty.type_definition().to_string()
                )
            }
            ReactiveRelationCreationError::InboundEntityDoesNotHaveComponent(relation_ty, component_ty) => {
                write!(f, "Relation type {} inbound relation instance doesn't have component {}", relation_ty, component_ty)
            }
            ReactiveRelationCreationError::InboundEntityIsNotOfType(expected_ty, actual_ty) => {
                write!(
                    f,
                    "Unknown relation type: {} (expected: {})",
                    expected_ty.type_definition().to_string(),
                    actual_ty.type_definition().to_string()
                )
            }
            ReactiveRelationCreationError::MissingOutboundEntityInstance(id) => {
                write!(f, "The outbound entity instance {id} cannot be found")
            }
            ReactiveRelationCreationError::MissingInboundEntityInstance(id) => {
                write!(f, "The inbound entity instance {id} cannot be found")
            }
            ReactiveRelationCreationError::MissingInstance(id) => {
                write!(f, "The created instance cannot be found with id {}", id)
            }
            ReactiveRelationCreationError::RelationInstanceCreationError(e) => {
                write!(f, "Failed to create reactive relation instance: {}", e)
            }
            ReactiveRelationCreationError::UnknownRelationType(ty) => {
                write!(f, "Unknown relation type: {}", ty.type_definition().to_string())
            }
            ReactiveRelationCreationError::ReactiveRelationRegistrationError(e) => {
                write!(f, "Registration Error: {:?}", e)
            }
        }
    }
}

#[derive(Debug)]
pub enum ReactiveRelationRegistrationError {
    /// The reactive relation instance cannot be created.
    RelationInstanceCreationError(RelationInstanceCreationError),
}

// #[derive(Debug)]
// pub enum ReactiveRelationImportError {
//     /// The reactive relation instance cannot be imported.
//     RelationInstanceImport(RelationInstanceImportError),
//     /// The reactive relation instance cannot be created.
//     ReactiveRelationCreation(ReactiveRelationCreationError),
// }

#[derive(Debug)]
pub enum ReactiveRelationComponentAddError {
    /// The given component doesn't exist.
    MissingComponent(ComponentTypeId),
    /// No reactive relation instance with the given edge key exists.
    MissingInstance(RelationInstanceId),
}

#[derive(Debug)]
pub enum ReactiveRelationPropertyAddError {
    /// No reactive relation instance with the given edge key exists.
    MissingInstance(RelationInstanceId),
    /// The property with the given name already exists.
    PropertyAlreadyExists(String),
}

#[derive(Debug)]
pub enum ReactiveRelationPropertyRemoveError {
    /// The property with the given name doesn't exist in the given relation instance.
    MissingProperty(String),
    /// No reactive entity relation with the given edge key exists.
    MissingInstance(RelationInstanceId),
    /// The property with the given name is in use by a component.
    PropertyInUseByComponent(ComponentTypeId),
}

#[async_trait]
pub trait ReactiveRelationManager: Send + Sync {
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
    fn create(&self, id: &RelationInstanceId, properties: PropertyInstances) -> Result<ReactiveRelation, ReactiveRelationCreationError>;

    fn create_reactive_instance(&self, relation_instance: RelationInstance) -> Result<ReactiveRelation, ReactiveRelationCreationError>;

    /// Registers the given reactive relation instance and applies components and behaviours.
    fn register_reactive_instance(
        &self,
        relation_instance: ReactiveRelation,
    ) -> Result<ReactiveRelation, ReactiveRelationRegistrationError>;

    fn register_or_merge_reactive_instance(
        &self,
        relation_instance: ReactiveRelation,
    ) -> Result<ReactiveRelation, ReactiveRelationRegistrationError>;

    /// Adds the component with the given name to the relation instance with the given edge key.
    fn add_component(&self, id: &RelationInstanceId, component_ty: &ComponentTypeId) -> Result<(), ReactiveRelationComponentAddError>;

    /// Removes the component with the given name from the relation instance with the given edge key.
    fn remove_component(&self, id: &RelationInstanceId, component_ty: &ComponentTypeId);

    /// Adds the property with the given name and initial value to the relation instance with the given id.
    fn add_property(
        &self,
        id: &RelationInstanceId,
        property_name: &str,
        mutability: Mutability,
        value: Value,
    ) -> Result<(), ReactiveRelationPropertyAddError>;

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
