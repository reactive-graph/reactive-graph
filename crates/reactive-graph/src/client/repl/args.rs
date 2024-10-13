use clap::Parser;

use crate::client::commands::ClientCommands;

#[derive(Clone, Debug, Parser)]
pub struct ReplArgs {
    #[command(subcommand)]
    pub(crate) commands: ClientCommands,
}
