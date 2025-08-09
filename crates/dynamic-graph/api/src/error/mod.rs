use async_graphql::dynamic::SchemaError;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::FlowTypeId;
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
pub enum PropertyArgumentMissingError {
    #[error("The property argument {0} was missing!")]
    PropertyArgumentMissing(String),
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

#[derive(Debug, Error)]
#[error("Can't find flow instance with id {0}")]
pub struct FlowInstanceNotFound(pub Uuid);

#[derive(Debug, Error)]
#[error("The flow instance {0} is not of entity type {2} which is defined in flow type {1}")]
pub struct FlowInstanceIsNotOfType(pub Uuid, pub FlowTypeId, pub EntityTypeId);
