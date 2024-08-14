use std::sync::Arc;

use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NoContent;
use crate::cli::error::CommandError::NotCreated;
use crate::cli::error::CommandError::NotFound;
use crate::cli::output_format::OutputFormatWrapper;
use crate::cli::result::CommandResult;
use crate::cli::types::components::args::ComponentsArgs;
use crate::cli::types::components::commands::ComponentsCommands;
use crate::table_model;
use crate::table_model::types::component::ComponentsTableOptions;
use reactive_graph_client::InexorRgfClient;

pub(crate) mod args;
pub(crate) mod commands;

type ComponentsOutputFormatWrapper = OutputFormatWrapper<reactive_graph_graph::Component, table_model::types::component::Component, ComponentsTableOptions>;

pub(crate) async fn components(client: &Arc<InexorRgfClient>, component_args: ComponentsArgs) -> CommandResult {
    let output_format_wrapper: ComponentsOutputFormatWrapper = component_args.output_format.into();
    let Some(command) = component_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        ComponentsCommands::List => match client.types().components().get_all_components().await {
            Ok(Some(components)) => output_format_wrapper.collection(components),
            Ok(None) => Err(NoContent("No components found".to_string())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::Get(args) => match client.types().components().get_component_by_type(args.clone()).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::Create(args) => match client.types().components().create_component_with_variables((&args).into()).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(NotCreated("Component wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::Delete(args) => match client.types().components().delete_component_with_variables((&args).into()).await {
            Ok(Some(true)) => Ok(format!("Component {}__{} deleted", args.namespace, args.name).into()),
            Ok(Some(false)) => Ok(format!("Component {}__{} not deleted", args.namespace, args.name).into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::AddProperty(args) => match client.types().components().add_property_with_variables((&args).into()).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(NotCreated("Property wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::RemoveProperty(args) => match client.types().components().remove_property_with_variables((&args).into()).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(NotFound(format!("Component {}__{} not found", args.ty.namespace, args.ty.name))),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::AddExtension(args) => match client.types().components().add_extension_with_variables((&args).into()).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(NotCreated("Extension wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::RemoveExtension(args) => match client.types().components().remove_extension_with_variables((&args).into()).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(args.ty.not_found()),
            Err(e) => Err(e.into()),
        },
        ComponentsCommands::UpdateDescription(args) => match client.types().components().update_description_with_variables((&args).into()).await {
            Ok(Some(component)) => output_format_wrapper.single(component),
            Ok(None) => Err(args.ty.not_found()),
            Err(e) => Err(e.into()),
        },
    }
}
