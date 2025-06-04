use crate::client::error::CommandError;
use crate::client::error::CommandError::NoContent;
use crate::client::error::CommandError::NotCreated;
use crate::client::error::CommandError::NotFound;
use crate::client::result::CommandResult;
use crate::client::types::extension::output_format::ExtensionsOutputFormatWrapper;
use crate::client::types::flows::args::FlowTypesArgs;
use crate::client::types::flows::commands::FlowTypesCommands;
use crate::client::types::flows::output_format::FlowTypesOutputFormatWrapper;
use crate::client::types::property_type::output_format::PropertyTypesOutputFormatWrapper;
use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_graph::NamespacedTypeContainer;
use std::sync::Arc;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod output_format;

pub(crate) async fn flow_types(client: &Arc<ReactiveGraphClient>, flow_type_args: FlowTypesArgs) -> CommandResult {
    let output_format_wrapper: FlowTypesOutputFormatWrapper = flow_type_args.output_format.clone().into();
    let Some(command) = flow_type_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        FlowTypesCommands::List => match client.types().flows().get_all_flow_types().await {
            Ok(Some(flow_types)) => output_format_wrapper.collection(flow_types),
            Ok(None) => Err(NoContent("No flow types found".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::Get(args) => match client.types().flows().get_flow_type_by_type(args.clone()).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::ListVariables(args) => match client.types().flows().get_flow_type_by_type(args.clone()).await {
            Ok(Some(flow_type)) => {
                let output_format_wrapper: PropertyTypesOutputFormatWrapper = flow_type_args.output_format.into();
                output_format_wrapper.collection(flow_type.variables.to_vec())
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::ListExtensions(args) => match client.types().flows().get_flow_type_by_type(args.clone()).await {
            Ok(Some(flow_type)) => {
                let output_format_wrapper: ExtensionsOutputFormatWrapper = flow_type_args.output_format.into();
                output_format_wrapper.collection(flow_type.extensions.to_vec())
            }
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::GetJsonSchema(args) => match client.types().flows().json_schema_for_flow_type_by_type(args.clone()).await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::Create(args) => match client.types().flows().create_flow_type_with_variables((&args).into()).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotCreated("Flow type wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::Delete(args) => match client.types().flows().delete_flow_type_with_variables((&args).into()).await {
            Ok(Some(true)) => Ok(format!("Flow type {}__{} deleted", args.namespace, args.name).into()),
            Ok(Some(false)) => Ok(format!("Flow type {}__{} not deleted", args.namespace, args.name).into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::AddVariable(args) => match client.types().flows().add_variable_with_variables((&args).into()).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotCreated("Property wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::RemoveVariable(args) => match client.types().flows().remove_variable_with_variables((&args).into()).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotFound(format!("Flow type {}__{} not found", args.ty.namespace, args.ty.name))),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::AddExtension(args) => match client.types().flows().add_extension_with_variables((&args).into()).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotCreated("Extension wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::RemoveExtension(args) => match client.types().flows().remove_extension_with_variables((&args).into()).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(args.flow_ty.not_found()),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::AddEntityInstance(args) => match client.types().flows().add_entity_instance_with_variables((&args).into()).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotCreated("Entity instance wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::RemoveEntityInstance(args) => match client.types().flows().remove_entity_instance_with_variables((&args).into()).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotCreated("Entity instance wasn't removed".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::UpdateDescription(args) => match client.types().flows().update_description_with_variables((&args).into()).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(args.ty.not_found()),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::JsonSchema => match client.json_schema().types().flows().await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NotFound("JSON Schema not available".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}
