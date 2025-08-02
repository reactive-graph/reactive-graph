use crate::tooling::releases::args::ReleaseInfoArgs;
use crate::tooling::releases::args::ReleaseListArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum InstallPluginsFromRepositoryCommands {
    /// Shows information about the selected release.
    Info(ReleaseInfoArgs),
    /// Lists the releases.
    List(ReleaseListArgs),
}
