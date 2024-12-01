use crate::client::instances::flows::commands::FlowInstancesCommands;
use crate::shared::output_format::OutputFormatArgs;
use clap::Args;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct FlowInstancesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<FlowInstancesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}
