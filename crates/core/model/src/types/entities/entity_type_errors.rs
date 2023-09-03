use thiserror::Error;

use crate::AddExtensionError;
use crate::AddPropertyError;
use crate::ComponentTypeId;
use crate::EntityTypeId;
use crate::RemoveExtensionError;
use crate::RemovePropertyError;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;

#[derive(Debug, Error)]
pub enum EntityTypeMergeError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
}

#[derive(Debug, Error)]
pub enum EntityTypeAddComponentError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
    #[error("The entity type is already a component {0}")]
    IsAlreadyA(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum EntityTypeUpdateComponentError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("The entity type is not a {0}")]
    IsNotA(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum EntityTypeRemoveComponentError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("The entity type is not a {0}")]
    IsNotA(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum EntityTypeAddPropertyError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("Failed to add property {0}")]
    AddPropertyError(#[from] AddPropertyError),
}

#[derive(Debug, Error)]
pub enum EntityTypeUpdatePropertyError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("Failed to update property {0}")]
    UpdatePropertyError(#[from] UpdatePropertyError),
}

#[derive(Debug, Error)]
pub enum EntityTypeRemovePropertyError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("Failed to remove property {0}")]
    RemovePropertyError(#[from] RemovePropertyError),
}

#[derive(Debug, Error)]
pub enum EntityTypeMergePropertiesError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
}

#[derive(Debug, Error)]
pub enum EntityTypeAddExtensionError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("Failed to add extension {0}")]
    AddExtensionError(#[from] AddExtensionError),
}

#[derive(Debug, Error)]
pub enum EntityTypeUpdateExtensionError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("Failed to update extension {0}")]
    UpdateExtensionError(#[from] UpdateExtensionError),
}

#[derive(Debug, Error)]
pub enum EntityTypeRemoveExtensionError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
    #[error("Failed to remove extension {0}")]
    RemoveExtensionError(#[from] RemoveExtensionError),
}

#[derive(Debug, Error)]
pub enum EntityTypeMergeExtensionsError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
}
