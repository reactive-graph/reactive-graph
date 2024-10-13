use crate::tooling::args::ToolingArguments;
use crate::tooling::commands::ToolingCommands;
use std::process::exit;

pub mod args;
pub mod commands;
pub mod install;
pub mod instances;

#[tokio::main]
pub async fn tooling(args: ToolingArguments) {
    if let Some(commands) = args.commands {
        match commands {
            ToolingCommands::Install(_args) => {
                // TODO: implement
                exit(0);
            }
            ToolingCommands::Instances(_args) => {
                // TODO: implement
                exit(0);
            }
        }
    }
}
