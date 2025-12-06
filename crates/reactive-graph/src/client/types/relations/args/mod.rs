use crate::client::types::relations::commands::RelationTypesCommands;
use crate::shared::output_format::OutputFormatArgs;
use clap::Args;
use reactive_graph_graph::InstanceId;
use reactive_graph_graph::RelationTypeId;
use std::error::Error;
use std::str::FromStr;

pub mod add_extension;
pub mod add_property;
pub mod create;
pub mod relation_component_type;
pub mod relation_extension_type;
pub mod relation_type_property;
pub mod update_description;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct RelationTypesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<RelationTypesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}

pub fn parse_relation_ty(namespace: &str) -> Result<RelationTypeId, Box<dyn Error + Send + Sync + 'static>> {
    Ok(RelationTypeId::from_str(namespace).map_err(Box::new)?)
}

pub fn parse_instance_id(instance_id: &str) -> Result<InstanceId, Box<dyn Error + Send + Sync + 'static>> {
    Ok(InstanceId::from_str(instance_id).map_err(Box::new)?)
}
