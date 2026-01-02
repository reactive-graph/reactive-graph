use crate::TypeDefinitionParseError;
use crate::TypeIdType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeIdParseError {
    #[error("Failed to parse type id: {0}")]
    TypeDefinitionParseError(TypeDefinitionParseError),
    #[error("The type id type must be {0} but was {1}")]
    InvalidTypeIdType(TypeIdType, TypeIdType),
}

#[derive(Debug, Error)]
pub enum TypeIdTypeParseError {
    #[error("The type id type {0} is unknown")]
    UnknownTypeIdType(String),
}
