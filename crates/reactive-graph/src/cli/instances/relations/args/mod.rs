use crate::cli::instances::relations::commands::RelationInstancesCommands;
use crate::cli::output_format::OutputFormatArgs;
use clap::Args;

pub mod search;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct RelationInstancesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<RelationInstancesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}
