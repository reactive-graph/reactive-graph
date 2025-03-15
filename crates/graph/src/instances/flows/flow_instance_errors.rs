use crate::EntityTypeId;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CreateFlowInstanceError {
    #[error("Can't get the wrapper entity {0}")]
    CantGetWrapperEntity(Uuid),
}

#[derive(Debug, Error)]
pub enum InvalidFlowInstanceError {
    #[error("Flow instance is invalid because the entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
}
