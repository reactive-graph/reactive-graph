use crate::client::types::entities::commands::EntityTypesCommands;
use crate::shared::output_format::OutputFormatArgs;
use clap::Args;

pub mod add_extension;
pub mod add_property;
pub mod create;
pub mod entity_component_type;
pub mod entity_extension_type;
pub mod entity_type_property;
pub mod type_id;
pub mod update_description;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct EntityTypesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<EntityTypesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}
