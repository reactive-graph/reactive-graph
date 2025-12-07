use crate::client::error::CommandError;
use crate::client::error::CommandError::NamespacedTypeNotFound;
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
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeGetter;
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
            Ok(Some(flow_types)) => output_format_wrapper.collection(flow_types.to_vec()),
            Ok(None) => Err(NoContent("No flow types found".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::Get { flow_ty } => match client.types().flows().get_flow_type_by_type(&flow_ty).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NamespacedTypeNotFound(flow_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::ListVariables { flow_ty } => match client.types().flows().get_flow_type_by_type(&flow_ty).await {
            Ok(Some(flow_type)) => {
                let output_format_wrapper: PropertyTypesOutputFormatWrapper = flow_type_args.output_format.into();
                output_format_wrapper.collection(flow_type.variables.to_vec())
            }
            Ok(None) => Err(NamespacedTypeNotFound(flow_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::ListExtensions { flow_ty } => match client.types().flows().get_flow_type_by_type(&flow_ty).await {
            Ok(Some(flow_type)) => {
                let output_format_wrapper: ExtensionsOutputFormatWrapper = flow_type_args.output_format.into();
                output_format_wrapper.collection(flow_type.extensions.to_vec())
            }
            Ok(None) => Err(NamespacedTypeNotFound(flow_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::GetJsonSchema { flow_ty } => match client.types().flows().json_schema_for_flow_type_by_type(&flow_ty).await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NamespacedTypeNotFound(flow_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::Create(args) => match client
            .types()
            .flows()
            .create_flow_type(
                FlowType::builder()
                    .ty(&args.flow_ty)
                    .description(args.description.unwrap_or_default())
                    .wrapper_entity_instance(
                        EntityInstance::builder()
                            .ty(&args.entity_ty)
                            .id(args.wrapper_entity_instance_id)
                            .description(args.wrapper_entity_instance_description.unwrap_or_default())
                            .build(),
                    )
                    .build(),
            )
            .await
        {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotCreated("Flow type wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::Delete { flow_ty } => match client.types().flows().delete_flow_type(&flow_ty).await {
            Ok(Some(true)) => Ok(format!("Flow Type {} deleted", flow_ty).into()),
            Ok(Some(false)) => Ok(format!("Flow Type {} not deleted", flow_ty).into()),
            Ok(None) => Err(NamespacedTypeNotFound(flow_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::AddVariable(args) => match client.types().flows().add_variable(&args.flow_ty, &args.variable).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotCreated("Property wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::RemoveVariable(args) => match client.types().flows().remove_variable(&args.flow_ty, args.variable_name.clone()).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NamespacedTypeNotFound(args.flow_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::AddExtension(args) => match client.types().flows().add_extension(&args.flow_ty, &args.extension).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotCreated("Extension wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::RemoveExtension(args) => match client.types().flows().remove_extension(&args).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NamespacedTypeNotFound(args.flow_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::AddEntityInstance(args) => match client.types().flows().add_entity_instance(&args.flow_ty, &args.entity_instance).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotCreated("Entity instance wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::RemoveEntityInstance(args) => match client.types().flows().remove_entity_instance(&args.flow_ty, args.id).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NotCreated("Entity instance wasn't removed".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::UpdateDescription(args) => match client.types().flows().update_description(&args.flow_ty, args.description).await {
            Ok(Some(flow_type)) => output_format_wrapper.single(flow_type),
            Ok(None) => Err(NamespacedTypeNotFound(args.flow_ty.namespaced_type())),
            Err(e) => Err(e.into()),
        },
        FlowTypesCommands::JsonSchema => match client.json_schema().types().flows().await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NotFound("JSON Schema not available".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}
