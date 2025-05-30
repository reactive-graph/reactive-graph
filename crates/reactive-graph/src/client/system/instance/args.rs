use clap::Args;

use crate::client::system::instance::commands::InstanceInfoCommands;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct InstanceInfoArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<InstanceInfoCommands>,
}
