use crate::tooling::instances::plugins::commands::PluginsCommands;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct PluginsArgs {
    #[command(subcommand)]
    pub commands: PluginsCommands,
}
