use thiserror::Error;

use crate::AddExtensionError;
use crate::AddPropertyError;
use crate::ComponentTypeId;
use crate::EntityComponentTypeIds;
use crate::EntityTypeId;
use crate::InvalidComponentError;
use crate::InvalidExtensionError;
use crate::InvalidPropertyTypeError;
use crate::NamespacedTypeParseError;
use crate::RemoveExtensionError;
use crate::RemovePropertyError;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;

#[derive(Debug, Error)]
pub enum EntityTypeUpdateError {
    #[error("The entity type {0} does not exist")]
    EntityTypeDoesNotExist(EntityTypeId),
}

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
pub enum EntityTypeMergeComponentPropertiesError {
    #[error("Missing components {0:?} while merging properties from components into an entity type")]
    ComponentDoesNotExist(EntityComponentTypeIds),
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

#[derive(Debug, Error)]
pub enum InvalidEntityTypeError {
    #[error("The fully qualified namespace of the entity type is invalid: {0}")]
    InvalidEntityType(#[from] NamespacedTypeParseError),
    #[error("The property type of the entity type is invalid: {0}")]
    InvalidPropertyType(InvalidPropertyTypeError),
    #[error("The component of the entity type is invalid: {0}")]
    InvalidComponent(#[from] InvalidComponentError),
    #[error("The extension of the entity type is invalid: {0}")]
    InvalidExtension(#[from] InvalidExtensionError),
}
