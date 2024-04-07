use clap::Parser;

use crate::cli::commands::ClientCommands;

#[derive(Clone, Debug, Parser)]
pub struct ReplArgs {
    #[command(subcommand)]
    pub(crate) commands: ClientCommands,
}
