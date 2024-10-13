use crate::tooling::instances::commands::InstancesCommands;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct InstancesArgs {
    // #[clap(flatten)]
    // pub connection: ClientConnectionArguments,
    #[command(subcommand)]
    pub commands: InstancesCommands,
}

#[derive(Parser, Debug)]
pub struct InitArgs {
    // #[clap(flatten)]
    // pub connection: ClientConnectionArguments,
}
