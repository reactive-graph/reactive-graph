use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum RemotesCommands {
    /// Lists the instance information of the remote instances.
    #[non_exhaustive]
    List,
}
