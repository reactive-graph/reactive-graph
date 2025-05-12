use std::sync::Arc;

use crate::client::error::CommandError;
use crate::client::introspection::args::IntrospectionQueryArgs;
use crate::client::introspection::commands::IntrospectionQueryCommands;
use crate::client::result::CommandResult;
use reactive_graph_client::ReactiveGraphClient;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn introspection_query(client: &Arc<ReactiveGraphClient>, args: IntrospectionQueryArgs) -> CommandResult {
    let Some(command) = args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        IntrospectionQueryCommands::ReactiveGraph => match client.introspection_query_reactive_graph().await {
            Ok(schema) => Ok(schema.to_sdl().into()),
            Err(e) => Err(e.into()),
        },
        IntrospectionQueryCommands::DynamicGraph => match client.introspection_query_dynamic_graph().await {
            Ok(schema) => Ok(schema.to_sdl().into()),
            Err(e) => Err(e.into()),
        },
        IntrospectionQueryCommands::ReactiveGraphRuntime => match client.introspection_query_reactive_graph_runtime().await {
            Ok(schema) => Ok(schema.to_sdl().into()),
            Err(e) => Err(e.into()),
        },
        IntrospectionQueryCommands::ReactiveGraphPlugins => match client.introspection_query_reactive_graph_plugins().await {
            Ok(schema) => Ok(schema.to_sdl().into()),
            Err(e) => Err(e.into()),
        },
    }
}
