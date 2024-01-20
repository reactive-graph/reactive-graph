use std::sync::Arc;

use async_graphql::*;
use uuid::Uuid;

use crate::api::CommandManager;
use crate::api::InstanceService;
use crate::api::PluginContainerManager;
use crate::api::RemotesManager;
use crate::graphql::query::system::GraphQLPlugin;
use crate::graphql::query::GraphQLCommand;
use crate::graphql::query::GraphQLInstanceInfo;
use crate::plugins::PluginState;

#[derive(Default)]
pub struct System;

#[Object]
impl System {
    async fn plugins(
        &self,
        context: &Context<'_>,
        id: Option<Uuid>,
        stem: Option<String>,
        name: Option<String>,
        state: Option<String>,
        has_dependencies: Option<bool>,
        has_unsatisfied_dependencies: Option<bool>,
    ) -> Result<Vec<GraphQLPlugin>> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        let plugins = plugin_container_manager
            .get_plugins()
            .into_iter()
            .filter(|plugin_id| match &id {
                Some(id) => plugin_id == id,
                None => true,
            })
            .filter(|plugin_id| match &stem {
                Some(stem) => match plugin_container_manager.get_id(stem.as_ref()) {
                    Some(id) => plugin_id == &id,
                    None => false,
                },
                None => true,
            })
            .filter(|plugin_id| match &name {
                Some(name) => match plugin_container_manager.name(plugin_id) {
                    Some(plugin_name) => &plugin_name == name,
                    None => false,
                },
                None => true,
            })
            .filter(|plugin_id| match &state {
                Some(state) => match plugin_container_manager.get_plugin_state(plugin_id) {
                    Some(PluginState::Installed) => state == "Installed",
                    Some(PluginState::Resolving(_)) => state == "Resolving",
                    Some(PluginState::Resolved) => state == "Resolved",
                    Some(PluginState::Starting(_)) => state == "Starting",
                    Some(PluginState::Active) => state == "Active",
                    Some(PluginState::Stopping(_)) => state == "Stopping",
                    Some(PluginState::Refreshing(_)) => state == "Refreshing",
                    Some(PluginState::Uninstalling(_)) => state == "Uninstalling",
                    Some(PluginState::Uninstalled) => state == "Uninstalled",
                    Some(PluginState::Disabled) => state == "Disabled",
                    None => false,
                },
                None => true,
            })
            .filter(|plugin_id| match &has_dependencies {
                Some(true) => plugin_container_manager.has_dependencies(plugin_id),
                Some(false) => !plugin_container_manager.has_dependencies(plugin_id),
                None => true,
            })
            .filter(|plugin_id| match &has_unsatisfied_dependencies {
                Some(true) => plugin_container_manager.has_unsatisfied_dependencies(plugin_id),
                Some(false) => !plugin_container_manager.has_unsatisfied_dependencies(plugin_id),
                None => true,
            })
            .map(|id| GraphQLPlugin { id })
            .collect();
        Ok(plugins)
    }

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
                        if name == command_name {
                            Some(GraphQLCommand { command })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                None => Some(GraphQLCommand { command }),
            })
            .collect())
    }
}
