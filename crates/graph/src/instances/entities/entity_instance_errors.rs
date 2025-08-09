use crate::EntityTypeId;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CreateEntityInstanceError {
    #[error("Cannot create entity instance of non-existing type {0}")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("Cannot create entity instance because one or multiple components does not exist")]
    ComponentsDoesNotExist,
}

#[derive(Debug, Error)]
pub enum AddEntityInstanceError {
    #[error("The entity instance {0} already exists")]
    EntityInstanceAlreadyExist(Uuid),
}

#[derive(Debug, Error)]
pub enum UpdateEntityInstanceError {
    #[error("The entity instance {0} does not exist")]
    EntityInstanceDoesNotExist(Uuid),
    #[error("No entity instance with label {0} exists")]
    EntityInstanceWithLabelDoesNotExist(String),
    #[error("Either the UUID or the label of the entity instance must be given")]
    EitherUuidOrLabelMustBeGiven,
}

#[derive(Debug, Error)]
pub enum TriggerEntityInstanceError {
    #[error("Unable to trigger entity instance {0} because there is no property trigger")]
    TriggerPropertyMissing(Uuid),
}

#[derive(Debug, Error)]
pub enum SubscribeEntityInstanceError {
    #[error("The entity instance {0} does not exist")]
    EntityInstanceDoesNotExist(Uuid),
    #[error("No entity instance with label {0} exists")]
    EntityInstanceWithLabelDoesNotExist(String),
    #[error("Either the UUID or the label of the entity instance must be given")]
    EitherUuidOrLabelMustBeGiven,
    #[error("Unable to subscribe non-existing property {1} of entity instance {0}")]
    PropertyNotFound(Uuid, String),
}

// #[derive(Debug, Error)]
// pub enum BehaviourTransitionEntityInstanceError {
//     #[error("Unable to trigger entity instance {0} because there is no property trigger")]
//     TriggerPropertyMissing(Uuid),
// }

#[derive(Debug, Error)]
pub enum RemoveEntityInstanceError {
    #[error("The entity instance {0} does not exist")]
    EntityInstanceDoesNotExist(Uuid),
    #[error("The entity instance {0} is in use")]
    EntityInstanceInUse(Uuid),
}
