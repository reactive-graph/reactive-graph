use crate::NamespacedTypeError;
use crate::TypeDefinition;
use crate::TypeIdType;
use crate::TypeIdTypeParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeDefinitionConversionError {
    #[error("The type definition {0} has type id type {1} but the target type id type is {2}")]
    TypeIdTypeMatchError(TypeDefinition, TypeIdType, TypeIdType),
}

#[derive(Debug, Error)]
pub enum TypeDefinitionParseError {
    #[error("The type id type is missing")]
    MissingTypeIdType,
    #[error("The type id type is invalid: {0}")]
    TypeIdTypeParseError(#[from] TypeIdTypeParseError),
    #[error("The namespace is missing")]
    MissingNamespace,
    #[error("The namespaced type is invalid: {0}")]
    NamespacedTypeError(#[from] NamespacedTypeError),
    #[error("JSON is not a valid type definition")]
    ParseJsonError,
}
