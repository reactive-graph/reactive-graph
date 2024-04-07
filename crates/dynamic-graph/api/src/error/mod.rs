use async_graphql::dynamic::SchemaError;
use reactive_graph_graph::EntityTypeId;
use serde_json::Error;
use thiserror::Error;
use uuid::Uuid;

use reactive_graph_graph::DataType;

#[derive(Debug, Error)]
pub enum DynamicQueryError {
    #[error("Failed to generate the dynamic schema!")]
    DynamicSchemaFailure(SchemaError),
    #[error("Error in JSON: {0}")]
    JsonError(#[from] Error),
}

#[derive(Debug, Error)]
pub enum PropertyDataTypeError {
    #[error("Null is not a valid datatype for property {0}!")]
    NullIsNotAValidDataType(String),
    #[error("Cannot set property {0} because value is of data type {1} but data type {2} is expected!")]
    ValueIsNotOfTheExpectedDataType(String, DataType, DataType),
}

#[derive(Debug, Error)]
#[error("Can't update immutable property {0}")]
pub struct ImmutablePropertyError(pub String);

#[derive(Debug, Error)]
#[error("Can't find entity instance with id {0}")]
pub struct EntityInstanceNotFound(pub Uuid);

#[derive(Debug, Error)]
#[error("The entity instance {0} is not of type {1}")]
pub struct EntityInstanceIsNotOfType(pub Uuid, pub EntityTypeId);
