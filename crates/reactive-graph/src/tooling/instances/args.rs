// use crate::server::args::graphql_server::GraphQLServerArguments;
use crate::tooling::instances::commands::InstancesCommands;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct InstancesArgs {
    /// The working directory of the instance.
    pub working_directory: Option<PathBuf>,

    #[command(subcommand)]
    pub commands: InstancesCommands,
}

#[derive(Parser, Debug)]
pub struct InitInstanceArgs {
    /// The name of the instance.
    #[arg(short = 'n', long)]
    pub name: Option<String>,

    /// The description of the instance.
    #[arg(short = 'd', long)]
    pub description: Option<String>,
}
