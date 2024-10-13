use crate::client::output_format::OutputFormatArgs;
use crate::client::types::relations::commands::RelationTypesCommands;
use clap::Args;

pub mod add_extension;
pub mod add_property;
pub mod create;
pub mod relation_component_type;
pub mod relation_extension_type;
pub mod relation_type_property;
pub mod type_id;
pub mod update_description;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct RelationTypesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<RelationTypesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}
