use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum UpdateCommands {
    /// Shows information about the selected release.
    Info,
    /// Lists the releases.
    List,
}
