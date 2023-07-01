use clap::Args;

use crate::cli::system::remotes::commands::RemotesCommands;

#[derive(Args, Debug, Clone)]
pub(crate) struct RemotesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<RemotesCommands>,
}
