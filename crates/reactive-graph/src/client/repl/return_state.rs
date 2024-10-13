use std::fmt::Display;
use std::fmt::Formatter;

use colored::Colorize;

use crate::client::repl::CHAR_ERROR;
use crate::client::repl::CHAR_SUCCESS;

pub enum ReturnState {
    Neutral,
    Success,
    Error,
}

impl Display for ReturnState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ReturnState::Neutral => write!(f, " "),
            ReturnState::Success => write!(f, "{}", CHAR_SUCCESS.green().bold()),
            ReturnState::Error => write!(f, "{}", CHAR_ERROR.red().bold()),
        }
    }
}
