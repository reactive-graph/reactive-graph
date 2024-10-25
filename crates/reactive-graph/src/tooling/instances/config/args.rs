use crate::tooling::instances::config::commands::InstanceConfigCommands;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ConfigInstanceArgs {
    #[command(subcommand)]
    pub commands: InstanceConfigCommands,
}
