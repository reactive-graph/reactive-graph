use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use reactive_graph_client::InexorRgfClientExecutionError;

#[derive(Debug)]
pub(crate) enum CommandError {
    MissingSubCommand,
    InexorRgfClientExecutionError(InexorRgfClientExecutionError),
    Rejected(String),
    NotFound(String),
    NoChange(String),
    NoContent(String),
    NotCreated(String),
}

impl CommandError {
    pub(crate) fn exit_code(&self) -> i32 {
        match self {
            CommandError::MissingSubCommand => 254,
            CommandError::InexorRgfClientExecutionError(_) => 253,
            CommandError::Rejected(_) => 4,
            CommandError::NotFound(_) => 3,
            CommandError::NoChange(_) => 2,
            CommandError::NoContent(_) => 1,
            CommandError::NotCreated(_) => 5,
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::MissingSubCommand => {
                writeln!(f, "Missing sub command")
            }
            CommandError::InexorRgfClientExecutionError(e) => {
                writeln!(f, "{e}")
            }
            CommandError::Rejected(message) => {
                writeln!(f, "{}", message)
            }
            CommandError::NotFound(message) => {
                writeln!(f, "{}", message)
            }
            CommandError::NoChange(message) => {
                writeln!(f, "{}", message)
            }
            CommandError::NoContent(message) => {
                writeln!(f, "{}", message)
            }
            CommandError::NotCreated(message) => {
                writeln!(f, "{}", message)
            }
        }
    }
}

impl Error for CommandError {
    fn description(&self) -> &str {
        match self {
            CommandError::MissingSubCommand => "Missing sub command",
            CommandError::InexorRgfClientExecutionError(_) => "Client execution error",
            CommandError::Rejected(message) => message,
            CommandError::NotFound(message) => message,
            CommandError::NoChange(message) => message,
            CommandError::NoContent(message) => message,
            CommandError::NotCreated(message) => message,
        }
    }
}

impl From<InexorRgfClientExecutionError> for CommandError {
    fn from(e: InexorRgfClientExecutionError) -> Self {
        CommandError::InexorRgfClientExecutionError(e)
    }
}
