use crate::RelationInstanceId;
use crate::RelationTypeId;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CreateRelationInstanceError {
    #[error("Cannot create relation instance of non-existing type {0}")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("Cannot create relation instance of type {0} because outbound entity instance {1} does not exist")]
    OutboundEntityInstanceDoesNotExist(RelationTypeId, Uuid),
    #[error("Cannot create relation instance of type {0} because inbound entity instance {1} does not exist")]
    InboundEntityInstanceDoesNotExist(RelationTypeId, Uuid),
    #[error("Cannot create relation instance because one or multiple components does not exist")]
    ComponentsDoesNotExist,
}
#[derive(Debug, Error)]
pub enum CreatePropertyConnectorError {
    #[error("Cannot create connector because {0}")]
    CreateRelationInstanceError(#[from] CreateRelationInstanceError),
    #[error("Cannot create connector of type {0} because outbound entity instance {1} has no property {2}")]
    OutboundPropertyDoesNotExist(RelationTypeId, Uuid, String),
    #[error("Cannot create connector of type {0} because inbound entity instance {1} has no property {2}")]
    InboundPropertyDoesNotExist(RelationTypeId, Uuid, String),
}

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
    #[error("Cannot create relation instance of non-existing type {0}")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("The outbound entity instance {0} does not exist")]
    OutboundEntityInstanceDoesNotExist(Uuid),
    #[error("The inbound entity instance {0} does not exist")]
    InboundEntityInstanceDoesNotExist(Uuid),
    #[error("The relation instance {0} does not exist")]
    RelationInstanceDoesNotExist(RelationInstanceId),
}

#[derive(Debug, Error)]
pub enum TriggerRelationInstanceError {
    #[error("The relation instance {0} does not exist")]
    RelationInstanceDoesNotExist(RelationInstanceId),
    #[error("Unable to trigger relation instance {0} because there is no property trigger")]
    TriggerPropertyMissing(RelationInstanceId),
}

#[derive(Debug, Error)]
pub enum SubscribeRelationInstanceError {
    #[error("The relation instance {0} does not exist")]
    RelationInstanceDoesNotExist(RelationInstanceId),
    #[error("Unable to subscribe non-existing property {1} of relation instance {0}")]
    PropertyNotFound(RelationInstanceId, String),
}

#[derive(Debug, Error)]
pub enum RemoveRelationInstanceError {
    #[error("The relation instance {0} is in use")]
    RelationInstanceDoesNotExist(RelationInstanceId),
    #[error("The outbound entity instance {0} does not exist")]
    OutboundEntityInstanceDoesNotExist(Uuid),
    #[error("The inbound entity instance {0} does not exist")]
    InboundEntityInstanceDoesNotExist(Uuid),
}
