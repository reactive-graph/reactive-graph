use crate::client::types::entities::commands::EntityTypesCommands;
use crate::shared::output_format::OutputFormatArgs;
use clap::Args;
use reactive_graph_graph::EntityTypeId;
use std::error::Error;
use std::str::FromStr;

pub mod add_extension;
pub mod add_property;
pub mod create;
pub mod entity_component_type;
pub mod entity_extension_type;
pub mod entity_type_property;
pub mod update_description;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct EntityTypesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<EntityTypesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}

pub fn parse_entity_ty(namespace: &str) -> Result<EntityTypeId, Box<dyn Error + Send + Sync + 'static>> {
    Ok(EntityTypeId::from_str(namespace).map_err(Box::new)?)
}
