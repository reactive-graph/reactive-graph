use crate::InstanceIdError;
use crate::InvalidComponentError;
use crate::NamespacedTypeParseError;
use crate::RelationInstanceId;
use crate::RelationTypeId;
use crate::TypeDefinitionParseError;
use crate::TypeIdParseError;
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

#[derive(Debug, Error)]
pub enum RelationInstanceTypeIdParseError {
    #[error("Failed to parse namespace: {0}")]
    NamespacedTypeParseError(#[from] NamespacedTypeParseError),
    #[error("Failed to parse instance id: {0}")]
    InstanceIdError(#[from] InstanceIdError),
    #[error("Failed to parse type definition: {0}")]
    TypeDefinitionParseError(#[from] TypeDefinitionParseError),
    #[error("Failed to parse type id: {0}")]
    TypeIdParseError(#[from] TypeIdParseError),
}

#[derive(Debug, Error)]
pub enum RelationInstanceTypeIdError {
    #[error("Failed to construct relation instance type id because of an error with the instance id: {0}")]
    InstanceIdError(#[from] InstanceIdError),
    #[error("Failed to construct relation instance type id because of an error with the namespaced type: {0}")]
    NamespacedTypeParseError(#[from] NamespacedTypeParseError),
}

#[derive(Debug, Error)]
pub enum RelationInstanceIdParseError {
    #[error("The outbound id is missing")]
    MissingOutboundId,
    #[error("The outbound id {0} is invalid")]
    InvalidOutboundId(String),
    #[error("Failed to parse relation instance type id: {0}")]
    RelationInstanceTypeIdParseError(#[from] RelationInstanceTypeIdParseError),
    #[error("The inbound id is missing")]
    MissingInboundId,
    #[error("The inbound id {0} is invalid")]
    InvalidInboundId(String),
}

#[derive(Debug, Error)]
pub enum InvalidRelationInstanceError {
    #[error("The relation instance type id is invalid: {0}")]
    InvalidRelationInstanceTypeId(#[from] RelationInstanceTypeIdError),
    #[error("The component of the relation instance is invalid: {0}")]
    InvalidComponent(#[from] InvalidComponentError),
    #[error("The relation instance is of non-existing type {0}")]
    RelationTypeDoesNotExist(RelationTypeId),
}
