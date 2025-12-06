use std::fmt::Debug;

use thiserror::Error;

use reactive_graph_client::ReactiveGraphClientExecutionError;
use reactive_graph_graph::NamespacedType;
use reactive_graph_serde::error::SerializationError;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Missing sub command")]
    MissingSubCommand,
    #[error("Execution failed: {0}")]
    ReactiveGraphClientExecutionError(ReactiveGraphClientExecutionError),
    #[error("Not yet implemented")]
    NotImplemented,
    #[error("Rejected: {0}")]
    Rejected(String),
    #[error("Type Not found: {0}")]
    NamespacedTypeNotFound(NamespacedType),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("No change: {0}")]
    NoChange(String),
    #[error("No content: {0}")]
    NoContent(String),
    #[error("Not created: {0}")]
    NotCreated(String),
    #[error("Not created: {0}")]
    NotRemoved(String),
    #[error("Serialization failed: {0}")]
    SerializationError(SerializationError),
}

impl CommandError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CommandError::MissingSubCommand => 254,
            CommandError::ReactiveGraphClientExecutionError(_) => 253,
            CommandError::NotImplemented => 252,
            CommandError::Rejected(_) => 4,
            CommandError::NamespacedTypeNotFound(_) => 7,
            CommandError::NotFound(_) => 3,
            CommandError::NoChange(_) => 2,
            CommandError::NoContent(_) => 1,
            CommandError::NotCreated(_) => 5,
            CommandError::NotRemoved(_) => 8,
            CommandError::SerializationError(_) => 6,
        }
    }
}

impl From<ReactiveGraphClientExecutionError> for CommandError {
    fn from(e: ReactiveGraphClientExecutionError) -> Self {
        CommandError::ReactiveGraphClientExecutionError(e)
    }
}

impl From<SerializationError> for CommandError {
    fn from(e: SerializationError) -> Self {
        CommandError::SerializationError(e)
    }
}
