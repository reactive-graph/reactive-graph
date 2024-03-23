use clap::error::ContextKind;
use clap::error::ContextValue;
use clap::error::ErrorKind;
use clap::CommandFactory;
use clap::Error;
use colored::Colorize;
use rustyline::hint::Hint;

use crate::cli::repl::args::ReplArgs;
use crate::cli::repl::chars::CHAR_ENTER;
use crate::cli::repl::chars::CHAR_ERROR;
use crate::cli::repl::chars::CHAR_SUGGESTION;
use crate::cli::repl::longest_common_prefix;

#[derive(Debug, Clone)]
pub enum HinterMatchType {
    Command(ReplArgs),
    Error(ErrorKind, Vec<(ContextKind, ContextValue)>),
}

#[derive(Debug, Clone)]
pub struct HinterMatch {
    display: String,
    completion: Option<String>,
}

impl HinterMatch {
    pub fn new_from_args_and_command(args: Vec<String>, command: ReplArgs) -> Self {
        HinterMatch::new(args, HinterMatchType::Command(command))
    }

    pub fn new_from_args_and_error(args: Vec<String>, error: Error) -> Self {
        HinterMatch::new(args, HinterMatchType::Error(error.kind(), error.context().map(|(k, v)| (k, v.clone())).collect()))
    }

    pub fn new(args: Vec<String>, t: HinterMatchType) -> Self {
        let last_arg = args.last().cloned().unwrap_or("".to_string());
        let display = if args.len() < 2 {
            let s: Vec<String> = ReplArgs::command().get_subcommands().map(|c| c.get_name().to_string()).collect();
            format!("   {} {}", CHAR_SUGGESTION, s.join(" | "))
        } else {
            match &t {
                HinterMatchType::Command(_command) => CHAR_ENTER.green().to_string(),
                HinterMatchType::Error(error_kind, context) => {
                    match context
                        .iter()
                        .find(|(kind, _)| kind != &ContextKind::InvalidSubcommand && kind != &ContextKind::Usage && kind != &ContextKind::Custom)
                        .and_then(|(kind, value)| match (kind, value) {
                            (ContextKind::InvalidArg, ContextValue::String(s)) => Some(format!("   {} {}", CHAR_SUGGESTION, s).to_string()),
                            (ContextKind::InvalidArg, ContextValue::Strings(s)) => Some(format!("   {} {}", CHAR_SUGGESTION, s.join(" | "))),
                            (ContextKind::SuggestedSubcommand, ContextValue::String(s)) => s.strip_prefix(&last_arg).map(|s| s.to_string()),
                            (ContextKind::SuggestedSubcommand, ContextValue::Strings(s)) => {
                                let mut s: Vec<String> = s.iter().filter_map(|s| s.strip_prefix(&last_arg).map(|s| s.to_string())).collect();
                                // s.sort_by(|s1, s2| s1.len().cmp(&s2.len()));
                                s.sort_by_key(|s| s.len());
                                Some(s.join(" | "))
                            }
                            (ContextKind::ValidSubcommand, ContextValue::String(s)) => Some(format!("   {} {}", CHAR_SUGGESTION, s)),
                            (ContextKind::ValidSubcommand, ContextValue::Strings(s)) => Some(format!("   {} {}", CHAR_SUGGESTION, s.join(" | "))),
                            (kind, value) => Some(format!(
                                "   {} |{}| : {} = {} ({})",
                                CHAR_ERROR,
                                error_kind,
                                kind,
                                value,
                                match value {
                                    ContextValue::None => "none",
                                    ContextValue::Bool(_) => "bool",
                                    ContextValue::String(_) => "string",
                                    ContextValue::Strings(_) => "strings",
                                    ContextValue::StyledStr(_) => "styledstr",
                                    ContextValue::StyledStrs(_) => "styledstrs",
                                    ContextValue::Number(_) => "number",
                                    _ => "unknown",
                                }
                            )),
                        }) {
                        Some(v) => v,
                        None => String::new(),
                    }
                }
            }
        };

        let mut error_info = String::new();

        let mut completion_matches = 0;
        let completion = match &t {
            HinterMatchType::Command(_command) => None,
            HinterMatchType::Error(error_kind, context) => context
                .iter()
                .find(|(kind, _)| kind != &ContextKind::InvalidSubcommand && kind != &ContextKind::Usage && kind != &ContextKind::Custom)
                .map(|(k, v)| {
                    error_info = format!("{} {} {}", error_info, CHAR_ERROR, error_kind.to_string().red());
                    (k, v)
                })
                .and_then(|(_, value)| match value {
                    ContextValue::String(s) => s.strip_prefix(&last_arg).map(|s| s.to_string()),
                    ContextValue::Strings(s) => {
                        let s: Vec<&String> = s.iter().filter(|s| s.starts_with(&last_arg)).collect();
                        completion_matches = s.len();
                        longest_common_prefix(&s).strip_prefix(&last_arg).map(|s| s.to_string())
                    }
                    _ => None,
                }),
        };

        let display = if !error_info.is_empty() {
            format!("{}  {}", display, error_info)
        } else {
            display
        };

        let completion = completion.map(|s| {
            let spacer = if completion_matches < 2 { " " } else { "" };
            format!("{}{}", s, spacer)
        });

        Self { display, completion }
    }
}

impl Hint for HinterMatch {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        self.completion.as_ref().map(|x| x.as_str())
    }
}
