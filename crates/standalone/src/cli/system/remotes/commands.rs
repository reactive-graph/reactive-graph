use crate::cli::system::remotes::args::InstanceAddressArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum RemotesCommands {
    /// Lists the remotes.
    #[non_exhaustive]
    List,

    /// Adds a remote.
    #[non_exhaustive]
    Add(InstanceAddressArgs),

    /// Removes a remote.
    #[non_exhaustive]
    Remove(InstanceAddressArgs),

    /// Removes all remotes.
    #[non_exhaustive]
    RemoveAll,

    /// Updates a remote.
    #[non_exhaustive]
    Update(InstanceAddressArgs),

    /// Updates all remotes.
    #[non_exhaustive]
    UpdateAll,

    /// Fetches the remotes from the given remote.
    #[non_exhaustive]
    FetchRemotesFromRemote(InstanceAddressArgs),

    /// Fetches all remotes from all remotes.
    #[non_exhaustive]
    FetchRemotesFromAllRemotes,
}
