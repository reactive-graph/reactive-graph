use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum InstanceInfoCommands {
    /// Get instance information.
    #[non_exhaustive]
    Get,
}
