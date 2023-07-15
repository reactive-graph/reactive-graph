use clap::Args;

use crate::cli::system::remotes::commands::RemotesCommands;
use crate::config::InstanceAddress;

#[derive(Args, Debug, Clone)]
pub(crate) struct RemotesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<RemotesCommands>,
}

#[derive(Args, Debug, Clone)]
pub(crate) struct InstanceAddressArgs {
    /// The hostname.
    #[arg(long)]
    pub hostname: String,

    /// The port.
    #[arg(long)]
    pub port: u16,

    /// The protocol.
    #[arg(long)]
    pub secure: Option<bool>,
}

impl From<InstanceAddressArgs> for InstanceAddress {
    fn from(address: InstanceAddressArgs) -> Self {
        InstanceAddress::new(address.hostname, address.port, address.secure.unwrap_or(false))
    }
}
