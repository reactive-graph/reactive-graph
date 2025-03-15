use crate::client::instances::flows::commands::FlowInstancesCommands;
use crate::shared::output_format::OutputFormatArgs;
use clap::Args;

pub mod create;
pub mod create_from_type;
pub mod id;
pub mod label;
pub mod search;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct FlowInstancesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<FlowInstancesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}
