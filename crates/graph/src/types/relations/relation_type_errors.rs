use crate::AddExtensionError;
use crate::AddPropertyError;
use crate::ComponentTypeId;
use crate::RelationTypeId;
use crate::RemoveExtensionError;
use crate::RemovePropertyError;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelationTypeMergeError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
}

#[derive(Debug, Error)]
pub enum RelationTypeAddComponentError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
    #[error("The relation type is already a component {0}")]
    IsAlreadyA(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum RelationTypeUpdateComponentError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("The relation type is not a {0}")]
    IsNotA(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum RelationTypeRemoveComponentError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("The relation type is not a {0}")]
    IsNotA(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum RelationTypeAddPropertyError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("Failed to add property {0}")]
    AddPropertyError(AddPropertyError),
}

#[derive(Debug, Error)]
pub enum RelationTypeUpdatePropertyError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("Failed to update property {0}")]
    UpdatePropertyError(UpdatePropertyError),
}

#[derive(Debug, Error)]
pub enum RelationTypeRemovePropertyError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("Failed to remove property {0}")]
    RemovePropertyError(RemovePropertyError),
}

#[derive(Debug, Error)]
pub enum RelationTypeMergePropertiesError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
}

#[derive(Debug, Error)]
pub enum RelationTypeAddExtensionError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("Failed to add extension {0}")]
    AddExtensionError(AddExtensionError),
}

#[derive(Debug, Error)]
pub enum RelationTypeUpdateExtensionError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("Failed to update extension {0}")]
    UpdateExtensionError(UpdateExtensionError),
}

#[derive(Debug, Error)]
pub enum RelationTypeRemoveExtensionError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
    #[error("Failed to remove extension {0}")]
    RemoveExtensionError(RemoveExtensionError),
}

#[derive(Debug, Error)]
pub enum RelationTypeMergeExtensionsError {
    #[error("The relation type {0} does not exist")]
    RelationTypeDoesNotExist(RelationTypeId),
}
