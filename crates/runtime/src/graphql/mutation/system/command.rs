use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::*;
use inexor_rgf_graph::EntityTypeId;

use crate::api::CommandManager;
use crate::graphql::query::GraphQLPropertyInstance;
use crate::model::PropertyTypeDefinition;
use crate::model_command::component::CommandProperties::COMMAND_RESULT;

#[derive(Default)]
pub struct MutationCommands;

/// Mutations for plugins.
#[Object]
impl MutationCommands {
    async fn execute(&self, context: &Context<'_>, name: String, args: Option<HashMap<String, Value>>) -> Result<Option<GraphQLPropertyInstance>> {
        let command_manager = context.data::<Arc<dyn CommandManager>>()?;
        let command = command_manager.get_command(&name).map_err(|_| Error::new("No such command"))?;
        let convert_result = convert_result(command.ty());
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

fn convert_result(ty: EntityTypeId) -> impl FnOnce(serde_json::Value) -> GraphQLPropertyInstance {
    |result: serde_json::Value| GraphQLPropertyInstance::new_entity_property(ty, COMMAND_RESULT.property_name(), result)
}
