use crate::client::types::flows::commands::FlowTypesCommands;
use crate::shared::output_format::OutputFormatArgs;
use clap::Args;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct FlowTypesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<FlowTypesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}
