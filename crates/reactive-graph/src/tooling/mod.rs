use crate::tooling::args::ToolingArguments;
use crate::tooling::commands::ToolingCommands;
use crate::tooling::instances::handle_instance;
use crate::tooling::update::handle_update;
use anyhow::Result;

pub mod args;
pub mod commands;
pub mod instances;
pub mod update;

pub fn tooling(args: ToolingArguments) -> Result<()> {
    if let Some(commands) = args.commands {
        match commands {
            ToolingCommands::Instances(args) => handle_instance(args)?,
            ToolingCommands::Update(args) => handle_update(args)?,
        }
    }
    Ok(())
}
