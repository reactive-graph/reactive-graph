use clap::Args;

use crate::client::system::remotes::commands::RemotesCommands;
use reactive_graph_remotes_model::InstanceAddress;
use reactive_graph_remotes_model::DEFAULT_PORT;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct RemotesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<RemotesCommands>,
}

#[derive(Args, Debug, Clone)]
pub(crate) struct InstanceAddressArgs {
    /// The hostname.
    #[arg(long, required = true)]
    pub hostname: String,

    /// The port.
    #[arg(long)]
    pub port: Option<u16>,

    /// The protocol.
    #[arg(long)]
    pub secure: Option<bool>,
}

impl From<InstanceAddressArgs> for InstanceAddress {
    fn from(address: InstanceAddressArgs) -> Self {
        InstanceAddress::new(address.hostname, address.port.unwrap_or(DEFAULT_PORT), address.secure.unwrap_or(false))
    }
}
