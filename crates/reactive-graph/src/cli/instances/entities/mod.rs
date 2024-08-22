use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NoContent;
use crate::cli::instances::entities::args::EntityInstancesArgs;
use crate::cli::instances::entities::commands::EntityInstancesCommands;
use crate::cli::instances::entities::output_format::EntityInstancesOutputFormatWrapper;
use crate::cli::result::CommandResult;
use reactive_graph_client::InexorRgfClient;
use std::sync::Arc;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod output_format;

pub(crate) async fn entity_instances(client: &Arc<InexorRgfClient>, entity_instances_args: EntityInstancesArgs) -> CommandResult {
    let output_format_wrapper: EntityInstancesOutputFormatWrapper = entity_instances_args.output_format.clone().into();
    let Some(command) = entity_instances_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        EntityInstancesCommands::List => match client.instances().entity_instances().get_all_entity_instances().await {
            Ok(Some(entity_instances)) => output_format_wrapper.collection(entity_instances),
            Ok(None) => Err(NoContent("No entity types found".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::Get(args) => match client.instances().entity_instances().get_entity_instance_by_id(args.clone()).await {
            Ok(Some(entity_instance)) => output_format_wrapper.single(entity_instance),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::GetByLabel(args) => match client.instances().entity_instances().get_entity_instance_by_label(args.label.clone()).await {
            Ok(Some(entity_instance)) => output_format_wrapper.single(entity_instance),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityInstancesCommands::Delete(args) => match client.instances().entity_instances().delete_entity_instance(args.id).await {
            Ok(Some(true)) => Ok(format!("Entity instance {} deleted", args.id).into()),
            Ok(Some(false)) => Ok(format!("Entity instance {} not deleted", args.id).into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
    }
}
