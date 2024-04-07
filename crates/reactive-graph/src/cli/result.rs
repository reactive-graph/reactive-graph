use std::fmt::Display;
use std::fmt::Formatter;

use serde_json::Value;
use tabled::Tabled;

use crate::cli::error::CommandError;
use crate::table_model::container::DefaultTableContainer;
use crate::table_model::container::TableContainer;
use crate::table_model::container::TableOptions;

pub(crate) enum CommandResponse {
    Message(String),
    Value(Value),
    Table(Box<dyn TableContainer>),
}

pub(crate) type CommandResult = Result<CommandResponse, CommandError>;

impl From<String> for CommandResponse {
    fn from(message: String) -> Self {
        CommandResponse::Message(message)
    }
}

impl From<&str> for CommandResponse {
    fn from(message: &str) -> Self {
        CommandResponse::Message(message.to_string())
    }
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        CommandResponse::Value(value)
    }
}

impl<S: 'static, T: Clone + Tabled + From<S> + 'static, O: TableOptions + 'static> From<DefaultTableContainer<S, T, O>> for CommandResponse {
    fn from(t: DefaultTableContainer<S, T, O>) -> Self {
        CommandResponse::Table(t.into_boxed())
    }
}

impl Display for CommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResponse::Message(message) => write!(f, "{}", message),
            CommandResponse::Value(value) => write!(f, "{}", serde_json::to_string_pretty(&value).unwrap_or_default()),
            CommandResponse::Table(table) => write!(f, "{}", table.table()),
        }
    }
}
