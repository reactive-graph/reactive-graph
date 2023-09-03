use thiserror::Error;
use crate::AddExtensionError;
use crate::AddPropertyError;
use crate::ComponentTypeId;
use crate::RemoveExtensionError;
use crate::RemovePropertyError;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;

#[derive(Debug, Error)]
pub enum AddComponentError {
    #[error("The component {0} already exists")]
    ComponentAlreadyExist(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum UpdateComponentError {
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum RemoveComponentError {
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum ComponentMergeError {
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
}

#[derive(Debug, Error)]
pub enum ComponentAddPropertyError {
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
    #[error("Failed to add property {0}")]
    AddPropertyError(#[from] AddPropertyError),
}

#[derive(Debug, Error)]
pub enum ComponentUpdatePropertyError {
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
    #[error("Failed to update property {0}")]
    UpdatePropertyError(#[from] UpdatePropertyError),
}

#[derive(Debug, Error)]
pub enum ComponentRemovePropertyError {
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
    #[error("Failed to remove property {0}")]
    RemovePropertyError(#[from] RemovePropertyError),
}

#[derive(Debug, Error)]
pub enum ComponentAddExtensionError {
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
    #[error("Failed to add extension {0}")]
    AddExtensionError(#[from] AddExtensionError),
}

#[derive(Debug, Error)]
pub enum ComponentUpdateExtensionError {
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
    #[error("Failed to update extension {0}")]
    UpdateExtensionError(#[from] UpdateExtensionError),
}

#[derive(Debug, Error)]
pub enum ComponentRemoveExtensionError {
    #[error("The component {0} does not exist")]
    ComponentDoesNotExist(ComponentTypeId),
    #[error("Failed to remove extension {0}")]
    RemoveExtensionError(#[from] RemoveExtensionError),
}
