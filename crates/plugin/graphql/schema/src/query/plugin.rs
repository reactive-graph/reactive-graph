use std::sync::Arc;

use async_graphql::*;
use reactive_graph_plugin_api::PLUGIN_NAME_PREFIX;
use reactive_graph_plugin_service_api::PluginContainerManager;
use uuid::Uuid;

pub struct GraphQLPlugin {
    pub id: Uuid,
}

#[Object(name = "Plugin")]
impl GraphQLPlugin {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn stem(&self, context: &Context<'_>) -> Result<String> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        plugin_container_manager
            .get_stem(&self.id)
            .ok_or_else(|| Error::new("Failed to resolve plugin stem"))
    }

    async fn path(&self, context: &Context<'_>) -> Result<String> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        plugin_container_manager
            .get_plugin_path(&self.id)
            .ok_or_else(|| Error::new("Failed to resolve plugin path"))
    }

    async fn state(&self, context: &Context<'_>) -> Result<String> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        plugin_container_manager
            .get_plugin_state(&self.id)
            .map(|s| format!("{:?}", s))
            .ok_or_else(|| Error::new("Failed to resolve plugin state"))
    }

    async fn name(&self, context: &Context<'_>) -> Result<String> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        plugin_container_manager
            .name(&self.id)
            .ok_or_else(|| Error::new("Failed to resolve plugin name"))
    }

    async fn short_name(&self, context: &Context<'_>) -> Result<String> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        plugin_container_manager
            .name(&self.id)
            .map(|name| name.replace(PLUGIN_NAME_PREFIX, ""))
            .ok_or_else(|| Error::new("Failed to resolve plugin short name"))
    }

    async fn description(&self, context: &Context<'_>) -> Result<String> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        plugin_container_manager
            .description(&self.id)
            .ok_or_else(|| Error::new("Failed to resolve plugin description"))
    }

    async fn version(&self, context: &Context<'_>) -> Result<String> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        plugin_container_manager
            .version(&self.id)
            .ok_or_else(|| Error::new("Failed to resolve plugin version"))
    }

    async fn rustc_version(&self, context: &Context<'_>) -> Result<String> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        plugin_container_manager
            .rustc_version(&self.id)
            .ok_or_else(|| Error::new("Failed to resolve plugin rustc_version"))
    }

    async fn plugin_api_version(&self, context: &Context<'_>) -> Result<String> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        plugin_container_manager
            .plugin_api_version(&self.id)
            .ok_or_else(|| Error::new("Failed to resolve plugin plugin_api_version"))
    }

    async fn dependencies(&self, context: &Context<'_>) -> Result<Vec<GraphQLPlugin>> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        let mut dependencies = Vec::new();
        for dependency in plugin_container_manager.get_dependencies(&self.id) {
            if let Some(dependency_id) = plugin_container_manager.get_plugin_by_dependency(&dependency) {
                dependencies.push(GraphQLPlugin { id: dependency_id })
            }
        }
        Ok(dependencies)
    }

    async fn unsatisfied_dependencies(&self, context: &Context<'_>) -> Result<Vec<GraphQLPlugin>> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        let mut dependencies = Vec::new();
        for dependency in plugin_container_manager.get_unsatisfied_dependencies(&self.id) {
            if let Some(dependency_id) = plugin_container_manager.get_plugin_by_dependency(&dependency) {
                dependencies.push(GraphQLPlugin { id: dependency_id })
            }
        }
        Ok(dependencies)
    }

    async fn dependents(&self, context: &Context<'_>) -> Result<Vec<GraphQLPlugin>> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        let mut dependents = Vec::new();
        for id in plugin_container_manager.get_dependents(&self.id) {
            dependents.push(GraphQLPlugin { id })
        }
        Ok(dependents)
    }
}
