use thiserror::Error;
use uuid::Uuid;
use crate::RelationInstanceId;

#[derive(Debug, Error)]
pub enum AddRelationInstanceError {
    #[error("The relation instance {0} already exists")]
    RelationInstanceAlreadyExist(RelationInstanceId),
    #[error("The outbound entity instance {0} does not exist")]
    OutboundEntityInstanceDoesNotExist(Uuid),
    #[error("The inbound entity instance {0} does not exist")]
    InboundEntityInstanceDoesNotExist(Uuid),
}

#[derive(Debug, Error)]
pub enum UpdateRelationInstanceError {
    #[error("The relation instance {0} does not exist")]
    RelationInstanceDoesNotExist(RelationInstanceId),
}

#[derive(Debug, Error)]
pub enum RemoveRelationInstanceError {
    #[error("The relation instance {0} is in use")]
    RelationInstanceDoesNotExist(RelationInstanceId),
}
