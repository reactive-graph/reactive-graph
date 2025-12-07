use crate::Namespace;
use crate::NamespaceError;
use crate::TypeDefinitionConversionError;
use crate::TypeDefinitionParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NamespacedTypeError {
    #[error("\"{0}\" is not a valid namespaced type because a type must be prefixed with a path")]
    MissingPathForType(Namespace),
    #[error("The namespace \"{0}\" must contain a type name as last segment. Type names must start with an uppercase letter.")]
    TypeNameMissing(Namespace),
    #[error("The namespaced type is not a valid namespace: {0}")]
    NamespaceError(#[from] NamespaceError),
}

#[derive(Debug, Error)]
pub enum NamespacedTypeParseError {
    #[error("Failed to parse namespace: {0}")]
    NamespacedTypeError(#[from] NamespacedTypeError),
    #[error("Failed to parse component type id: {0}")]
    TypeDefinitionParseError(#[from] TypeDefinitionParseError),
    #[error("Invalid type definition: {0}")]
    TypeDefinitionConversionError(#[from] TypeDefinitionConversionError),
}
