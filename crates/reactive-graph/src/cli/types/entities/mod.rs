use std::sync::Arc;

use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NoContent;
use crate::cli::error::CommandError::NotCreated;
use crate::cli::error::CommandError::NotFound;
use crate::cli::output_format::OutputFormatWrapper;
use crate::cli::result::CommandResult;
use crate::cli::types::entities::args::EntityTypesArgs;
use crate::cli::types::entities::commands::EntityTypesCommands;
use crate::table_model;
use crate::table_model::types::entity_type::EntityTypesTableOptions;
use reactive_graph_client::InexorRgfClient;

pub(crate) mod args;
pub(crate) mod commands;

type EntityTypesOutputFormatWrapper =
    OutputFormatWrapper<reactive_graph_graph::EntityType, table_model::types::entity_type::EntityType, EntityTypesTableOptions>;

pub(crate) async fn entity_types(client: &Arc<InexorRgfClient>, entity_type_args: EntityTypesArgs) -> CommandResult {
    let output_format_wrapper: EntityTypesOutputFormatWrapper = entity_type_args.output_format.into();
    let Some(command) = entity_type_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        EntityTypesCommands::List => match client.types().entity_types().get_all_entity_types().await {
            Ok(Some(entity_types)) => output_format_wrapper.collection(entity_types),
            Ok(None) => Err(NoContent("No entity types found".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Get(args) => match client.types().entity_types().get_entity_type_by_type(args.clone()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Create(args) => match client.types().entity_types().create_entity_type_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Entity type wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Delete(args) => match client.types().entity_types().delete_entity_type_with_variables((&args).into()).await {
            Ok(Some(true)) => Ok(format!("Entity type {}__{} deleted", args.namespace, args.name).into()),
            Ok(Some(false)) => Ok(format!("Entity type {}__{} not deleted", args.namespace, args.name).into()),
            Ok(None) => Err(args.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::AddProperty(args) => match client.types().entity_types().add_property_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Property wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::RemoveProperty(args) => match client.types().entity_types().remove_property_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotFound(format!("Entity type {}__{} not found", args.ty.namespace, args.ty.name))),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::AddExtension(args) => match client.types().entity_types().add_extension_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(NotCreated("Extension wasn't created".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::RemoveExtension(args) => match client.types().entity_types().remove_extension_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(args.ty.not_found()),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::UpdateDescription(args) => match client.types().entity_types().update_description_with_variables((&args).into()).await {
            Ok(Some(entity_type)) => output_format_wrapper.single(entity_type),
            Ok(None) => Err(args.ty.not_found()),
            Err(e) => Err(e.into()),
        },
    }
}
