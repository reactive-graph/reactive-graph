use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

use crate::api::EntityInstanceCreationError;
use crate::api::EntityInstanceImportError;
use crate::api::Lifecycle;
use crate::model::BehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::model::EntityInstance;
use crate::model::EntityTypeId;
use crate::model::ReactiveEntityInstance;

#[derive(Debug)]
pub enum ReactiveEntityInstanceCreationError {
    UuidTaken(Uuid),
    MissingInstance,
    EntityInstanceCreationError(EntityInstanceCreationError),
    ReactiveEntityInstanceRegistrationError(ReactiveEntityInstanceRegistrationError),
}

impl fmt::Display for ReactiveEntityInstanceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ReactiveEntityInstanceCreationError::UuidTaken(id) => {
                write!(f, "The UUID {} has been already taken!", id)
            }
            ReactiveEntityInstanceCreationError::MissingInstance => {
                write!(f, "The created instance cannot be found")
            }
            ReactiveEntityInstanceCreationError::EntityInstanceCreationError(e) => {
                write!(f, "Failed to create reactive entity instance: {}", e)
            }
            ReactiveEntityInstanceCreationError::ReactiveEntityInstanceRegistrationError(e) => {
                write!(f, "Failed to register reactive entity instance: {:?}", e)
            }
        }
    }
}

#[derive(Debug)]
pub enum ReactiveEntityInstanceRegistrationError {
    /// The reactive entity instance cannot be created.
    EntityInstanceCreationError(EntityInstanceCreationError),
}

#[derive(Debug)]
pub enum ReactiveEntityInstanceImportError {
    /// The reactive entity instance cannot be imported.
    EntityInstanceImport(EntityInstanceImportError),
    MissingEntityInstance(Uuid),
    /// The reactive entity instance cannot be created.
    ReactiveEntityInstanceCreation(ReactiveEntityInstanceCreationError),
}

#[derive(Debug)]
pub enum ReactiveEntityInstanceComponentAddError {
    /// The given component doesn't exist.
    MissingComponent(ComponentTypeId),
    /// No reactive entity instance with the given id exists.
    MissingInstance(Uuid),
}

#[derive(Debug)]
pub enum ReactiveEntityInstancePropertyAddError {
    /// No reactive entity instance with the given id exists.
    MissingInstance(Uuid),
    /// The property with the given name already exists.
    PropertyAlreadyExists(String),
}

#[derive(Debug)]
pub enum ReactiveEntityInstancePropertyRemoveError {
    /// The property with the given name doesn't exist in the given entity instance.
    MissingProperty(String),
    /// No reactive entity instance with the given id exists.
    MissingInstance(Uuid),
    /// The property with the given name is in use by a component.
    PropertyInUseByComponent(String, ComponentTypeId),
}

#[async_trait]
pub trait ReactiveEntityInstanceManager: Send + Sync + Lifecycle {
    /// Returns true, if an entity instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the reactive entity instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>>;

    /// Returns the reactive entity instance that matches the given label or None.
    fn get_by_label(&self, label: &str) -> Option<Arc<ReactiveEntityInstance>>;

    /// Returns the reactive entity instance and the matched path parameters that matches the given label or None.
    /// /org/inexor/local/users/:user_id
    /// /org/inexor/local/users/PeterPenacka returns: (instance, {"user_id": "PeterPenacka"})
    fn get_by_label_with_params(&self, label: &str) -> Option<(Arc<ReactiveEntityInstance>, HashMap<String, String>)>;

    /// Returns all registered reactive entity instances.
    fn get_all(&self) -> Vec<Arc<ReactiveEntityInstance>>;

    /// Returns all reactive entity instances of the given type.
    fn get_by_type(&self, ty: &EntityTypeId) -> Vec<Arc<ReactiveEntityInstance>>;

    /// Returns the ids of all registered reactive entity instances.
    fn get_ids(&self) -> Vec<Uuid>;

    /// Returns the count of registered reactive entity instances.
    fn count(&self) -> usize;

    /// Returns the count of registered reactive entity instances of the given type.
    fn count_by_type(&self, ty: &EntityTypeId) -> usize;

    /// Returns the count of registered reactive entity instances which are of the given component.
    fn count_by_component(&self, component_ty: &ComponentTypeId) -> usize;

    /// Returns the count of registered reactive entity instances which behaves as the given behaviour.
    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize;

    /// Creates a new reactive entity instance of the given type. The reactive instance will be
    /// initialized with the given properties and values. A random id will be generated.
    fn create(&self, ty: &EntityTypeId, properties: HashMap<String, Value>) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    /// Creates a new reactive entity instance of the given type, with the given id and initialized
    /// with the given properties and values.
    fn create_with_id(
        &self,
        ty: &EntityTypeId,
        id: Uuid,
        properties: HashMap<String, Value>,
    ) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    /// Creates a reactive entity instance from the given non-reactive entity instance. The
    /// reactive entity instance will be registered.
    fn create_reactive_instance(&self, entity_instance: EntityInstance) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    /// Registers a reactive entity instance and applies components and behaviours.
    fn register_reactive_instance(
        &self,
        reactive_entity_instance: Arc<ReactiveEntityInstance>,
    ) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceRegistrationError>;

    /// Registers a reactive entity instance if and only if the given instance doesn't exist.
    ///
    /// No properties are merged if the given entity instance already exists.
    fn register_or_merge_reactive_instance(
        &self,
        reactive_entity_instance: Arc<ReactiveEntityInstance>,
    ) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceRegistrationError>;

    /// Adds the component with the given name to the entity instance with the given id.
    fn add_component(&self, id: Uuid, component_ty: &ComponentTypeId) -> Result<(), ReactiveEntityInstanceComponentAddError>;

    /// Removes the component with the given name from the entity instance with the given id.
    fn remove_component(&self, id: Uuid, component_ty: &ComponentTypeId);

    /// Adds the property with the given name and initial value to the entity instance with the given id.
    fn add_property(&self, id: Uuid, property_name: &str, value: Value) -> Result<(), ReactiveEntityInstancePropertyAddError>;

    /// Removes the property with the given name from the entity instance with the given id.
    fn remove_property(&self, id: Uuid, property_name: &str) -> Result<(), ReactiveEntityInstancePropertyRemoveError>;

    // TODO: return result
    fn commit(&self, id: Uuid);

    fn delete(&self, id: Uuid);

    // TODO: fn delete_and_delete_relations(&self, id: Uuid);

    /// Unregisters the reactive entity instance. Also removes all behaviours. If there are any
    /// references to the reactive entity instance, their reactive streams still work but the
    /// applied behaviours are gone.
    fn unregister_reactive_instance(&self, id: Uuid);

    // TODO: rename import_from_file
    fn import(&self, path: &str) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceImportError>;

    // TODO: import_from_json_string
    // Goal: web-ui: upload entity instance as json file

    // TODO: return result
    // TODO: rename export_as_file
    fn export(&self, id: Uuid, path: &str);

    // TODO: implement export_as_json_string
    // Goal: web-ui: download entity instance as json file
    // fn export_as_json_string(&self, id: Uuid) -> String;

    fn handle_component_added_events(&self);

    fn handle_component_removed_events(&self);

    fn handle_property_added_events(&self);

    fn handle_property_removed_events(&self);
}
