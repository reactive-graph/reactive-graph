use crate::EntityTypeId;
use thiserror::Error;
use uuid::Uuid;

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
    #[error("Flow instance is invalid because the entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
}
