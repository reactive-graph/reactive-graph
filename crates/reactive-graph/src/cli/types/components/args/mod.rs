pub mod add_extension;
pub mod add_property;
pub mod component_extension_type;
pub mod component_property;
pub mod create;
pub mod type_id;
pub mod update_description;

use crate::cli::output_format::OutputFormatArgs;
use crate::cli::types::components::args::type_id::ComponentTypeIdArgs;
use crate::cli::types::components::commands::ComponentsCommands;
use clap::Args;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct ComponentsArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<ComponentsCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}
