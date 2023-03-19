use std::sync::Arc;

use async_graphql::*;
use uuid::Uuid;

use crate::api::InstanceService;
use crate::api::PluginContainerManager;
use crate::graphql::query::system::GraphQLPlugin;
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
    ) -> Result<Vec<GraphQLPlugin>> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager>>()?;
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
            .map(|id| GraphQLPlugin { id })
            .collect();
        Ok(plugins)
    }

    /// Returns the instance information.
    async fn instance_info(&self, context: &Context<'_>) -> Result<GraphQLInstanceInfo> {
        let instance_service = context.data::<Arc<dyn InstanceService>>()?;
        let instance_info = instance_service.get_instance_info();
        Ok(GraphQLInstanceInfo { instance_info })
    }
}
