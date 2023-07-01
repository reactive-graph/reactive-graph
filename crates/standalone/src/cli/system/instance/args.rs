use clap::Args;

use crate::cli::system::instance::commands::InstanceInfoCommands;

#[derive(Args, Debug, Clone)]
pub(crate) struct InstanceInfoArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<InstanceInfoCommands>,
}
