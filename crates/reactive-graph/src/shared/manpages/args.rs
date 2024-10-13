use crate::shared::manpages::commands::ManPagesActionCommands;
use crate::shared::manpages::commands::ManPagesCommands;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ManPagesArguments {
    #[command(subcommand)]
    pub(crate) commands: Option<ManPagesCommands>,
}

#[derive(Parser, Debug)]
pub struct ManPagesActionArgs {
    #[command(subcommand)]
    pub commands: ManPagesActionCommands,
}
