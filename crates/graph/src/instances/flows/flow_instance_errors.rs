use crate::EntityTypeId;
use crate::InvalidEntityInstanceError;
use crate::InvalidRelationInstanceError;
use crate::NamespacedTypeParseError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum QueryFlowInstanceError {
    #[error("The given uuid is invalid")]
    InvalidUuid(#[from] uuid::Error),
    #[error("The flow instance {0} does not exist")]
    FlowInstanceDoesNotExist(Uuid),
    #[error("No flow instance with label {0} exists")]
    FlowInstanceWithLabelDoesNotExist(String),
    #[error("The flow instance {0} is not of type {1}")]
    FlowInstanceIsNotOfType(Uuid, EntityTypeId),
}

#[derive(Debug, Error)]
pub enum CreateFlowInstanceError {
    #[error("Cannot create entity instance of non-existing type {0}")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("A flow with id {0} already exists")]
    FlowAlreadyExists(Uuid),
    #[error("An entity instance with id {0} already exists and cannot be used as wrapper entity instance")]
    WrapperEntityInstanceAlreadyExists(Uuid),
    #[error("Can't get the wrapper entity {0}")]
    CantGetWrapperEntity(Uuid),
}

#[derive(Debug, Error)]
pub enum InvalidFlowInstanceError {
    #[error("The entity type of the flow instance is invalid: {0}")]
    InvalidEntityType(#[from] NamespacedTypeParseError),
    #[error("The entity instance of the flow instance is invalid: {0}")]
    InvalidEntityInstance(#[from] InvalidEntityInstanceError),
    #[error("The relation instance of the flow instance is invalid: {0}")]
    InvalidRelationInstance(#[from] InvalidRelationInstanceError),
    #[error("The flow instance is invalid because the entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
}
