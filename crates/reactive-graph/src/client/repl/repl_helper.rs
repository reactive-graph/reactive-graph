use std::borrow::Cow;

use clap::Parser;
use colored::Colorize;
use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::ValidationContext;
use rustyline::validate::ValidationResult;
use rustyline::validate::Validator;
use rustyline::Context;
use rustyline::Helper;
use rustyline::Result;
use shellwords::split;

use crate::client::repl::args::ReplArgs;
use crate::client::repl::hint::HinterMatch;

pub struct ReplHelper {}

impl ReplHelper {
    pub fn new() -> Self {
        Self {}
    }
}

impl Completer for ReplHelper {
    type Candidate = &'static str;
}

impl Highlighter for ReplHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(hint.cyan().to_string())
    }
}

impl Validator for ReplHelper {
    fn validate(&self, _: &mut ValidationContext) -> Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Hinter for ReplHelper {
    type Hint = HinterMatch;

    fn hint(&self, line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        let Ok(args) = split(line).map(|mut args| {
            args.insert(0, String::from(" "));
            args
        }) else {
            return None;
        };
        let hinter_match = match ReplArgs::try_parse_from(args.clone()) {
            Ok(command) => HinterMatch::new_from_args_and_command(args, command),
            Err(error) => HinterMatch::new_from_args_and_error(args, error),
        };
        Some(hinter_match)
    }
}

impl Helper for ReplHelper {}
