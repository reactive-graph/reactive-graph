use crate::tooling::instances::plugins::install::commands::InstallPluginsFromRepositoryCommands;
use crate::tooling::releases::args::ReleaseArgs;
use crate::tooling::repository::args::RepositoryArgs;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(disable_version_flag = true)]
pub struct InstallPluginsFromRepositoryArgs {
    #[clap(flatten)]
    pub release: ReleaseArgs,

    #[clap(flatten)]
    pub repository: RepositoryArgs,

    #[command(subcommand)]
    pub commands: Option<InstallPluginsFromRepositoryCommands>,
}
