use crate::client::instances::relations::commands::RelationInstancesCommands;
use crate::client::output_format::OutputFormatArgs;
use clap::Args;

pub mod add_property;
pub mod create;
pub mod id;
pub mod id_and_component;
pub mod id_and_property;
pub mod search;
pub mod set_property;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct RelationInstancesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<RelationInstancesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}
