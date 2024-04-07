use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;
use async_graphql::Value;

use reactive_graph_command_api::CommandManager;
use reactive_graph_command_model::component::CommandProperties::COMMAND_RESULT;
use reactive_graph_graph::PropertyTypeDefinition;

use crate::properties::GraphQLCommandResult;

#[derive(Default)]
pub struct MutationCommands;

/// Mutations for plugins.
#[Object]
impl MutationCommands {
    async fn execute(&self, context: &Context<'_>, name: String, args: Option<HashMap<String, Value>>) -> Result<Option<GraphQLCommandResult>> {
        let command_manager = context.data::<Arc<dyn CommandManager + Send + Sync>>()?;
        let command = command_manager.get_command(&name).map_err(|_| Error::new("No such command"))?;
        let convert_result = convert_result();
        let result = match args.map(convert_args) {
            Some(args) => command.execute_with_args(args),
            None => command.execute(),
        }
        .map_err(|_| Error::new("Command execution failed"))?
        .map(convert_result);
        Ok(result)
    }
}

fn convert_args(args: HashMap<String, Value>) -> HashMap<String, serde_json::Value> {
    args.into_iter().filter_map(map_entry).collect()
}

fn map_entry(entry: (String, Value)) -> Option<(String, serde_json::Value)> {
    match entry.1.into_json() {
        Ok(v) => Some((entry.0, v)),
        Err(_) => None,
    }
}

fn convert_result() -> impl FnOnce(serde_json::Value) -> GraphQLCommandResult {
    |result: serde_json::Value| GraphQLCommandResult::new(COMMAND_RESULT.property_name(), result)
}
