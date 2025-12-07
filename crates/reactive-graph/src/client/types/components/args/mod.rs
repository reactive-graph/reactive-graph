pub mod add_extension;
pub mod add_property;
pub mod component_extension_type;
pub mod component_property;
pub mod create;
pub mod type_id;
pub mod update_description;

use crate::shared::output_format::OutputFormatArgs;

use crate::client::types::components::commands::ComponentsCommands;
use clap::Args;
use reactive_graph_graph::ComponentTypeId;
use std::error::Error;
use std::str::FromStr;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct ComponentsArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<ComponentsCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}

pub fn parse_component_ty(namespace: &str) -> Result<ComponentTypeId, Box<dyn Error + Send + Sync + 'static>> {
    Ok(ComponentTypeId::from_str(namespace).map_err(Box::new)?)
}
