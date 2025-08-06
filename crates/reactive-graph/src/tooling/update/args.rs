use crate::tooling::releases::args::ReleaseArgs;
use crate::tooling::repository::args::RepositoryArgs;
use crate::tooling::update::commands::UpdateCommands;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(disable_version_flag = true)]
pub struct UpdateArgs {
    #[clap(flatten)]
    pub repository: RepositoryArgs,

    #[clap(flatten)]
    pub release: ReleaseArgs,

    #[command(subcommand)]
    pub commands: Option<UpdateCommands>,
}
