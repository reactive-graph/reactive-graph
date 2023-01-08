use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use indradb::ValidationError;
use serde_json::Value;
use uuid::Uuid;

use crate::api::RelationInstanceCreationError;
use crate::api::RelationInstanceImportError;
use crate::model::BehaviourTypeId;
use crate::model::ComponentBehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::Mutability;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationBehaviourTypeId;
use crate::model::RelationInstance;
use crate::model::RelationTypeId;
use crate::model::TypeDefinitionGetter;

#[derive(Debug)]
pub enum ReactiveRelationInstanceCreationError {
    InvalidEdgeKey,
    OutboundEntityDoesNotHaveComponent(RelationTypeId, ComponentTypeId),
    OutboundEntityIsNotOfType(RelationTypeId, EntityTypeId),
    InboundEntityDoesNotHaveComponent(RelationTypeId, ComponentTypeId),
    InboundEntityIsNotOfType(RelationTypeId, EntityTypeId),
    MissingOutboundEntityInstance(Uuid),
    MissingInboundEntityInstance(Uuid),
    /// No reactive relation instance with the given edge key exists.
    MissingInstance(EdgeKey),
    RelationInstanceCreationError(RelationInstanceCreationError),
    UnknownRelationType(RelationTypeId),
    ValidationError(ValidationError),
    ReactiveRelationInstanceRegistrationError(ReactiveRelationInstanceRegistrationError),
}

impl fmt::Display for ReactiveRelationInstanceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            // ReactiveRelationInstanceCreationError::UuidTaken(id) => write!(f, "The UUID {} has been already taken!", id),
            ReactiveRelationInstanceCreationError::InvalidEdgeKey => {
                write!(f, "The edge key is invalid")
            }
            ReactiveRelationInstanceCreationError::OutboundEntityDoesNotHaveComponent(relation_ty, component_ty) => {
                write!(f, "Relation type {} outbound relation instance doesn't have component {}", relation_ty, component_ty)
            }
            ReactiveRelationInstanceCreationError::OutboundEntityIsNotOfType(expected_ty, actual_ty) => {
                write!(
                    f,
                    "Unknown relation type: {} (expected: {})",
                    expected_ty.type_definition().to_string(),
                    actual_ty.type_definition().to_string()
                )
            }
            ReactiveRelationInstanceCreationError::InboundEntityDoesNotHaveComponent(relation_ty, component_ty) => {
                write!(f, "Relation type {} inbound relation instance doesn't have component {}", relation_ty, component_ty)
            }
            ReactiveRelationInstanceCreationError::InboundEntityIsNotOfType(expected_ty, actual_ty) => {
                write!(
                    f,
                    "Unknown relation type: {} (expected: {})",
                    expected_ty.type_definition().to_string(),
                    actual_ty.type_definition().to_string()
                )
            }
            ReactiveRelationInstanceCreationError::MissingOutboundEntityInstance(id) => {
                write!(f, "The outbound entity instance {id} cannot be found")
            }
            ReactiveRelationInstanceCreationError::MissingInboundEntityInstance(id) => {
                write!(f, "The inbound entity instance {id} cannot be found")
            }
            ReactiveRelationInstanceCreationError::MissingInstance(edge_key) => {
                write!(f, "The created instance cannot be found with edge key {}", edge_key.t.as_str())
            }
            ReactiveRelationInstanceCreationError::RelationInstanceCreationError(e) => {
                write!(f, "Failed to create reactive relation instance: {}", e)
            }
            ReactiveRelationInstanceCreationError::UnknownRelationType(ty) => {
                write!(f, "Unknown relation type: {}", ty.type_definition().to_string())
            }
            ReactiveRelationInstanceCreationError::ValidationError(e) => {
                write!(f, "Validation Error: {}", e)
            }
            ReactiveRelationInstanceCreationError::ReactiveRelationInstanceRegistrationError(e) => {
                write!(f, "Registration Error: {:?}", e)
            }
        }
    }
}

#[derive(Debug)]
pub enum ReactiveRelationInstanceRegistrationError {
    /// The reactive relation instance cannot be created.
    RelationInstanceCreationError(RelationInstanceCreationError),
}

#[derive(Debug)]
pub enum ReactiveRelationInstanceImportError {
    /// The reactive relation instance cannot be imported.
    RelationInstanceImport(RelationInstanceImportError),
    /// The reactive relation instance cannot be created.
    ReactiveRelationInstanceCreation(ReactiveRelationInstanceCreationError),
}

#[derive(Debug)]
pub enum ReactiveRelationInstanceComponentAddError {
    /// The given component doesn't exist.
    MissingComponent(ComponentTypeId),
    /// No reactive relation instance with the given edge key exists.
    MissingInstance(EdgeKey),
}

#[derive(Debug)]
pub enum ReactiveRelationInstancePropertyAddError {
    /// No reactive relation instance with the given edge key exists.
    MissingInstance(EdgeKey),
    /// The property with the given name already exists.
    PropertyAlreadyExists(String),
}

#[derive(Debug)]
pub enum ReactiveRelationInstancePropertyRemoveError {
    /// The property with the given name doesn't exist in the given relation instance.
    MissingProperty(String),
    /// No reactive entity relation with the given edge key exists.
    MissingInstance(EdgeKey),
    /// The property with the given name is in use by a component.
    PropertyInUseByComponent(ComponentTypeId),
}

#[async_trait]
pub trait ReactiveRelationInstanceManager: Send + Sync {
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
    fn get_by_type(&self, ty: &RelationTypeId) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all reactive relation instances of the given type which are of the given component..
    fn get_by_component(&self, component_ty: &ComponentTypeId) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all reactive relation instances of the given type which behaves as the given behaviour.
    fn get_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all reactive relation instances of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> Vec<Arc<ReactiveRelationInstance>>;

    /// Returns all edge keys.
    fn get_keys(&self) -> Vec<EdgeKey>;

    /// Returns the count of registered reactive relation instances.
    fn count(&self) -> usize;

    /// Returns the count of registered reactive relation instances of the given type.
    fn count_by_type(&self, ty: &RelationTypeId) -> usize;

    /// Returns the count of registered reactive relation instances which are of the given component.
    fn count_by_component(&self, component_ty: &ComponentTypeId) -> usize;

    /// Returns the count of registered reactive relation instances which behaves as the given behaviour.
    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize;

    /// Creates a new reactive relation instance.
    fn create(&self, edge_key: &EdgeKey, properties: HashMap<String, Value>) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>;

    fn create_reactive_instance(&self, relation_instance: RelationInstance) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>;

    /// Registers the given reactive relation instance and applies components and behaviours.
    fn register_reactive_instance(
        &self,
        relation_instance: Arc<ReactiveRelationInstance>,
    ) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceRegistrationError>;

    fn register_or_merge_reactive_instance(
        &self,
        relation_instance: Arc<ReactiveRelationInstance>,
    ) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceRegistrationError>;

    /// Adds the component with the given name to the relation instance with the given edge key.
    fn add_component(&self, edge_key: &EdgeKey, component_ty: &ComponentTypeId) -> Result<(), ReactiveRelationInstanceComponentAddError>;

    /// Removes the component with the given name from the relation instance with the given edge key.
    fn remove_component(&self, edge_key: &EdgeKey, component_ty: &ComponentTypeId);

    /// Adds the property with the given name and initial value to the relation instance with the given id.
    fn add_property(
        &self,
        edge_key: &EdgeKey,
        property_name: &str,
        mutability: Mutability,
        value: Value,
    ) -> Result<(), ReactiveRelationInstancePropertyAddError>;

    /// Removes the property with the given name from the relation instance with the given id.
    fn remove_property(&self, edge_key: &EdgeKey, property_name: &str) -> Result<(), ReactiveRelationInstancePropertyRemoveError>;

    /// Adds the given behaviour to all instances of the given relation type.
    fn add_behaviour_to_all_relation_instances(&self, relation_behaviour_ty: &RelationBehaviourTypeId);

    /// Adds the given behaviour to all instances of the given relation type.
    fn add_behaviour_to_all_relation_components(&self, component_behaviour_ty: &ComponentBehaviourTypeId);

    // TODO: fn commit(&self, relation_instance: RelationInstance);
    // TODO: return result
    fn commit(&self, edge_key: &EdgeKey);

    /// Deletes the reactive relation instance with the given key.
    fn delete(&self, edge_key: &EdgeKey) -> bool;

    fn unregister_reactive_instance(&self, edge_key: &EdgeKey);

    fn import(&self, path: &str) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceImportError>;

    // TODO: return result
    fn export(&self, edge_key: &EdgeKey, path: &str);

    fn handle_component_added_events(&self);

    fn handle_component_removed_events(&self);

    fn handle_property_added_events(&self);

    fn handle_property_removed_events(&self);
}
