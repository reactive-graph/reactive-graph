use std::sync::Arc;

use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NoContent;
use crate::cli::error::CommandError::NotCreated;
use crate::cli::error::CommandError::NotFound;
use crate::cli::result::CommandResult;
use crate::cli::types::entities::args::EntityTypesArgs;
use crate::cli::types::entities::commands::EntityTypesCommands;
use crate::table_model::types::entity_type::EntityTypes;
use reactive_graph_client::types::entity_types::queries::CreateEntityTypeVariables;
use reactive_graph_client::InexorRgfClient;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn entity_types(client: &Arc<InexorRgfClient>, entity_type_args: EntityTypesArgs) -> CommandResult {
    let Some(command) = entity_type_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        EntityTypesCommands::List => match client.types().entity_types().get_all_entity_types().await {
            Ok(Some(entity_types)) => Ok(EntityTypes::from(entity_types).into()),
            Ok(None) => Err(NoContent("No entity_types found".to_string())),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Get(args) => match client.types().entity_types().get_entity_type_by_type(args.clone()).await {
            Ok(Some(entity_type)) => Ok(EntityTypes::from(entity_type).into()),
            Ok(None) => Err(NotFound(format!("EntityType {}__{} not found", args.namespace, args.name))),
            Err(e) => Err(e.into()),
        },
        EntityTypesCommands::Create(args) => {
            let variables = CreateEntityTypeVariables::builder()
                .namespace(args.ty.namespace)
                .name(args.ty.name)
                .description(args.description)
                // .properties(None)
                // .extensions(None)
                .build();
            match client.types().entity_types().create_entity_type_with_variables(variables).await {
                Ok(Some(entity_type)) => Ok(EntityTypes::from(entity_type).into()),
                Ok(None) => Err(NotCreated("EntityType wasn't created".to_string())),
                Err(e) => Err(e.into()),
            }
        }
    }
}
