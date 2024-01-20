use thiserror::Error;
use uuid::Uuid;

use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_graph::RelationTypeId;

#[derive(Debug, Error)]
pub enum ReactiveRelationCreationError {
    #[error("The edge key is invalid!")] // TODO: Rename or remove!
    InvalidEdgeKey,
    #[error("The outbound entity {0} doesn't have component {1}!")]
    OutboundEntityDoesNotHaveComponent(Uuid, ComponentTypeId),
    #[error("The outbound entity {0} is of type {1} but expected {2}!")]
    OutboundEntityIsNotOfType(Uuid, EntityTypeId, EntityTypeId),
    #[error("The inbound entity {0} doesn't have component {1}!")]
    InboundEntityDoesNotHaveComponent(Uuid, ComponentTypeId),
    #[error("The inbound entity {0} is of type {1} but expected {2}!")]
    InboundEntityIsNotOfType(Uuid, EntityTypeId, EntityTypeId),
    #[error("The outbound entity {0} doesn't exist!")]
    MissingOutboundEntityInstance(Uuid),
    #[error("The inbound entity {0} doesn't exist!")]
    MissingInboundEntityInstance(Uuid),
    #[error("The created reactive relation {0} wasn't found after creation!")]
    MissingInstance(RelationInstanceId),
    #[error("The relation type {0} is unknown!")]
    UnknownRelationType(RelationTypeId),
    // ValidationError(ValidationError),
    #[error("Failed to register the reactive relation: {0}")]
    ReactiveRelationRegistrationError(ReactiveRelationRegistrationError),
}

#[derive(Debug, Error)]
pub enum ReactiveRelationRegistrationError {
    #[error("Couldn't register reactive relation {0} because it already exists!")]
    RelationInstanceAlreadyExists(RelationInstanceId),
}

#[derive(Debug, Error)]
pub enum ReactiveRelationComponentAddError {
    #[error("The reactive relation {0} wasn't found!")]
    MissingInstance(RelationInstanceId),
    #[error("Couldn't add the unknown component {0} to the reactive relation!")]
    ComponentNotRegistered(ComponentTypeId),
    #[error("The reactive relation is already {0}!")]
    IsAlreadyA(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum ReactiveRelationComponentRemoveError {
    #[error("The reactive relation {0} wasn't found!")]
    MissingInstance(RelationInstanceId),
    #[error("Couldn't remove the unknown component {0}!")]
    ComponentNotRegistered(ComponentTypeId),
    #[error("The reactive relation is not a {0}!")]
    IsNotA(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum ReactiveRelationPropertyAddError {
    #[error("The reactive relation {0} wasn't found!")]
    MissingInstance(RelationInstanceId),
    #[error("The reactive relation already has property {0}!")]
    PropertyAlreadyExists(String),
}

#[derive(Debug, Error)]
pub enum ReactiveRelationPropertyRemoveError {
    #[error("The reactive relation {0} wasn't found!")]
    MissingInstance(RelationInstanceId),
    #[error("The reactive relation doesn't has a property {0}!")]
    MissingProperty(String),
    #[error("Cannot remove property {0} because it is still in use!")]
    PropertyInUseByComponent(ComponentTypeId),
}
