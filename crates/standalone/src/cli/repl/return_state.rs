use std::fmt::Display;
use std::fmt::Formatter;

use colored::Colorize;

use crate::cli::repl::CHAR_ERROR;
use crate::cli::repl::CHAR_SUCCESS;

pub enum ReturnState {
    NEUTRAL,
    SUCCESS,
    ERROR,
}

impl Display for ReturnState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ReturnState::NEUTRAL => write!(f, " "),
            ReturnState::SUCCESS => write!(f, "{}", CHAR_SUCCESS.green().bold()),
            ReturnState::ERROR => write!(f, "{}", CHAR_ERROR.red().bold()),
        }
    }
}
