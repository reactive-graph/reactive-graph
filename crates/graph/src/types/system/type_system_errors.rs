use crate::ComponentTypeId;
use crate::EntityTypeId;
use crate::EntityTypeMergeComponentPropertiesError;
use crate::FlowTypeId;
use crate::RelationTypeId;
use crate::RelationTypeMergeComponentPropertiesError;
use reactive_graph_serde::error::DeserializationError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeSystemMergeComponentPropertiesError {
    #[error("Failed to merge component properties of entity components: {0}")]
    EntityTypeComponentDoesNotExist(#[from] EntityTypeMergeComponentPropertiesError),
    #[error("Failed to merge component properties of relation components: {0}")]
    RelationTypeComponentDoesNotExist(#[from] RelationTypeMergeComponentPropertiesError),
}

#[derive(Debug, Error)]
pub enum TypeDefinitionImportError {
    #[error("The import path {0} doesn't exist.")]
    ImportPathDoesNotExist(PathBuf),
    #[error("Failed to import type definition: Path is a directory.")]
    DirEntryError(PathBuf),
    #[error("Failed to import type definition: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Can't import type definition from unknown file type {0} in {1}.")]
    UnsupportedFormat(String, PathBuf),
    #[error("Failed to deserialize: {0}.")]
    DeserializationError(#[from] DeserializationError),
}

#[derive(Debug, Error)]
pub enum TypeResolveError {
    #[error("Failed to resolve component {0}")]
    ComponentResolveError(ComponentTypeId),
    #[error("Failed to resolve entity type {0}")]
    EntityTypeResolveError(EntityTypeId),
    #[error("Failed to resolve relation type {0}")]
    RelationTypeResolveError(RelationTypeId),
    #[error("Failed to resolve flow type {0}")]
    FlowTypeResolveError(FlowTypeId),
}
