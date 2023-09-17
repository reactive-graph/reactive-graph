use thiserror::Error;
use uuid::Uuid;

use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;

#[derive(Debug, Error)]
pub enum ReactiveEntityCreationError {
    #[error("Cannot create reactive entity because uuid {0} is already taken!")]
    UuidTaken(Uuid),
    #[error("The created instance cannot be found!")]
    MissingInstance,
    #[error("Failed to register reactive entity instance: {0}")]
    ReactiveEntityRegistrationError(#[from] ReactiveEntityRegistrationError),
}

#[derive(Debug, Error)]
pub enum ReactiveEntityRegistrationError {
    #[error("Cannot register reactive entity because uuid {0} is already taken!")]
    UuidTaken(Uuid),
    #[error("Cannot register reactive entity because entity type {0} is unknown!")]
    UnknownEntityType(EntityTypeId),
}

#[derive(Debug, Error)]
pub enum ReactiveEntityComponentAddError {
    #[error("Cannot add non-existent component {0} to the reactive entity!")]
    MissingComponent(ComponentTypeId),
    #[error("The reactive entity with id {0} doesn't exist!")]
    MissingInstance(Uuid),
}

#[derive(Debug, Error)]
pub enum ReactiveEntityPropertyAddError {
    #[error("The reactive entity with id {0} doesn't exist!")]
    MissingInstance(Uuid),
    #[error("Cannot add property {0} to reactive entity because property already exist!")]
    PropertyAlreadyExists(String),
}

#[derive(Debug, Error)]
pub enum ReactiveEntityPropertyRemoveError {
    #[error("Cannot remove non-existent property {0} from reactive entity!")]
    MissingProperty(String),
    #[error("The reactive entity with id {0} doesn't exist!")]
    MissingInstance(Uuid),
    #[error("Cannot remove property {0} from reactive entity because it is in use by component {1}!")]
    PropertyInUseByComponent(String, ComponentTypeId),
}
