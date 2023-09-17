use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AddEntityInstanceError {
    #[error("The entity instance {0} already exists")]
    EntityInstanceAlreadyExist(Uuid),
}

#[derive(Debug, Error)]
pub enum UpdateEntityInstanceError {
    #[error("The entity instance {0} does not exist")]
    EntityInstanceDoesNotExist(Uuid),
}

#[derive(Debug, Error)]
pub enum RemoveEntityInstanceError {
    #[error("The entity instance {0} does not exist")]
    EntityInstanceDoesNotExist(Uuid),
    #[error("The entity instance {0} is in use")]
    EntityInstanceInUse(Uuid),
}
