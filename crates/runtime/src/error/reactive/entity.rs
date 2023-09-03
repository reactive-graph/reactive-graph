use std::fmt;

use uuid::Uuid;

use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;

#[derive(Debug)]
pub enum ReactiveEntityCreationError {
    UuidTaken(Uuid),
    MissingInstance,
    ReactiveEntityRegistrationError(ReactiveEntityRegistrationError),
}

impl fmt::Display for ReactiveEntityCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ReactiveEntityCreationError::UuidTaken(id) => {
                write!(f, "The UUID {} has been already taken!", id)
            }
            ReactiveEntityCreationError::MissingInstance => {
                write!(f, "The created instance cannot be found")
            }
            ReactiveEntityCreationError::ReactiveEntityRegistrationError(e) => {
                write!(f, "Failed to register reactive entity instance: {:?}", e)
            }
        }
    }
}

#[derive(Debug)]
pub enum ReactiveEntityRegistrationError {
    UuidTaken(Uuid),
    UnknownEntityType(EntityTypeId)
}

#[derive(Debug)]
pub enum ReactiveEntityComponentAddError {
    /// The given component doesn't exist.
    MissingComponent(ComponentTypeId),
    /// No reactive entity instance with the given id exists.
    MissingInstance(Uuid),
}

#[derive(Debug)]
pub enum ReactiveEntityPropertyAddError {
    /// No reactive entity instance with the given id exists.
    MissingInstance(Uuid),
    /// The property with the given name already exists.
    PropertyAlreadyExists(String),
}

#[derive(Debug)]
pub enum ReactiveEntityPropertyRemoveError {
    /// The property with the given name doesn't exist in the given entity instance.
    MissingProperty(String),
    /// No reactive entity instance with the given id exists.
    MissingInstance(Uuid),
    /// The property with the given name is in use by a component.
    PropertyInUseByComponent(String, ComponentTypeId),
}
