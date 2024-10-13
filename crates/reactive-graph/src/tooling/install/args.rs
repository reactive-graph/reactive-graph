use crate::tooling::install::commands::InstallCommands;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct InstallArgs {
    #[command(subcommand)]
    pub commands: InstallCommands,
}
