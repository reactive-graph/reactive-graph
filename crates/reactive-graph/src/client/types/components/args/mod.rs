pub mod add_extension;
pub mod add_property;
pub mod component_extension_type;
pub mod component_property;
pub mod create;
pub mod type_id;
pub mod update_description;

use crate::shared::output_format::OutputFormatArgs;

use crate::client::types::components::args::type_id::ComponentTypeIdArgs;
use crate::client::types::components::commands::ComponentsCommands;
use clap::Args;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::NAMESPACE_SEPARATOR;
use std::error::Error;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct ComponentsArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<ComponentsCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}

pub fn parse_component_ty(s: &str) -> Result<ComponentTypeId, Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find(NAMESPACE_SEPARATOR)
        .ok_or_else(|| format!("no namespace delimiter `{NAMESPACE_SEPARATOR}` found in `{s}`"))?;
    Ok(ComponentTypeId::new_from_type(s[..pos].to_string(), s[pos + 2..].to_string()))
}
