use crate::cli::instances::entities::commands::EntityInstancesCommands;
use crate::cli::output_format::OutputFormatArgs;
use clap::Args;

pub mod add_component;
pub mod add_property;
pub mod create;
pub mod id;
pub mod id_and_property;
pub mod label;
pub mod search;
pub mod set_property;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct EntityInstancesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<EntityInstancesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}
