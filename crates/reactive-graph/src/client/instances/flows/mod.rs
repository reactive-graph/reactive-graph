use crate::client::error::CommandError;
use crate::client::error::CommandError::NoContent;
use crate::client::error::CommandError::NotFound;
use crate::client::instances::flows::args::FlowInstancesArgs;
use crate::client::instances::flows::commands::FlowInstancesCommands;
use crate::client::instances::flows::output_format::FlowInstancesOutputFormatWrapper;
use crate::client::result::CommandResult;
use reactive_graph_client::ReactiveGraphClient;
use std::sync::Arc;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod output_format;

pub(crate) async fn flow_instances(client: &Arc<ReactiveGraphClient>, flow_instances_args: FlowInstancesArgs) -> CommandResult {
    let output_format_wrapper: FlowInstancesOutputFormatWrapper = flow_instances_args.output_format.clone().into();
    let Some(command) = flow_instances_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        FlowInstancesCommands::List(args) => match client.instances().flows().search((&args).into()).await {
            Ok(Some(flow_instances)) => output_format_wrapper.collection(flow_instances.to_vec()),
            Ok(None) => Err(NoContent("No flow instances found".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowInstancesCommands::Get(args) => match client.instances().flows().get_by_id(args.clone()).await {
            Ok(Some(flow_instance)) => output_format_wrapper.single(flow_instance),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        FlowInstancesCommands::GetByLabel(args) => match client.instances().flows().get_by_label(args.label.clone()).await {
            Ok(Some(flow_instance)) => output_format_wrapper.single(flow_instance),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        // FlowInstancesCommands::Create(args) => match client.instances().flows().create_from_type(args.id, args.description.clone(), args.properties()).await {
        //     Ok(Some(flow_instance)) => output_format_wrapper.single(flow_instance),
        //     Ok(None) => Err(NoContent("Flow instance not created".to_string())),
        //     Err(e) => Err(e.into()),
        // },
        FlowInstancesCommands::CreateFromType(args) => match client
            .instances()
            .flows()
            .create_from_type(args.flow_ty.clone(), args.id, args.variables(), args.properties())
            .await
        {
            Ok(Some(flow_instance)) => output_format_wrapper.single(flow_instance),
            Ok(None) => Err(NoContent("Flow instance not created".to_string())),
            Err(e) => Err(e.into()),
        },
        FlowInstancesCommands::Delete(args) => match client.instances().flows().delete(args.id).await {
            Ok(Some(true)) => Ok(format!("Flow instance {} deleted", args.id).into()),
            Ok(Some(false)) => Ok(format!("Flow instance {} not deleted", args.id).into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        FlowInstancesCommands::JsonSchema => match client.json_schema().instances().flows().await {
            Ok(Some(json_schema)) => Ok(json_schema.into()),
            Ok(None) => Err(NotFound("JSON Schema not available".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}
