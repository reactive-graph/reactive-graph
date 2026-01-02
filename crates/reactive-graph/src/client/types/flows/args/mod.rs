use crate::client::types::flows::commands::FlowTypesCommands;
use crate::shared::output_format::OutputFormatArgs;
use clap::Args;
use reactive_graph_graph::FlowTypeId;
use std::error::Error;
use std::str::FromStr;

pub mod add_entity_instance;
pub mod add_extension;
pub mod add_variable;
pub mod create;
pub mod flow_extension_type;
pub mod flow_type_variable;
pub mod remove_entity_instance;
// pub mod type_id;
pub mod update_description;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct FlowTypesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<FlowTypesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}

pub fn parse_flow_ty(namespace: &str) -> Result<FlowTypeId, Box<dyn Error + Send + Sync + 'static>> {
    Ok(FlowTypeId::from_str(namespace).map_err(Box::new)?)
}
