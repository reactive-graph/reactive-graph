use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_command_api::CommandManager;
use reactive_graph_remotes_api::InstanceService;
use reactive_graph_remotes_api::RemotesManager;

use crate::query::command::GraphQLCommand;
use crate::query::instance::GraphQLInstanceInfo;

pub mod command;
pub mod instance;

pub struct RuntimeQuery;

/// Search queries for the type system, the instances and the flows.
#[Object(name = "Query")]
impl RuntimeQuery {
    /// Returns the instance information.
    async fn instance_info(&self, context: &Context<'_>) -> Result<GraphQLInstanceInfo> {
        let instance_service = context.data::<Arc<dyn InstanceService + Send + Sync>>()?;
        let instance_info = instance_service.get_instance_info();
        Ok(GraphQLInstanceInfo { instance_info })
    }

    /// Returns the list of remotes.
    async fn remotes(&self, context: &Context<'_>) -> Result<Vec<GraphQLInstanceInfo>> {
        let remotes_manager = context.data::<Arc<dyn RemotesManager + Send + Sync>>()?;
        Ok(remotes_manager.get_all().into_iter().map(GraphQLInstanceInfo::from).collect())
    }

    /// Returns the commands.
    async fn commands(&self, context: &Context<'_>, name: Option<String>) -> Result<Vec<GraphQLCommand>> {
        let command_manager = context.data::<Arc<dyn CommandManager + Send + Sync>>()?;
        Ok(command_manager
            .get_commands()
            .into_iter()
            .filter_map(|command| match name.clone() {
                Some(name) => {
                    if let Some(command_name) = command.name() {
                        if name == command_name { Some(GraphQLCommand { command }) } else { None }
                    } else {
                        None
                    }
                }
                None => Some(GraphQLCommand { command }),
            })
            .collect())
    }
}
