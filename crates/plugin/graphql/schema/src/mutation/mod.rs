use std::sync::Arc;

use async_graphql::*;

use inexor_rgf_plugin_service_api::PluginContainerManager;
use inexor_rgf_plugin_service_api::PluginResolver;

use crate::query::GraphQLPlugin;

pub struct PluginMutation;

/// Mutations for the type system, the instances and the flows.
#[Object(name = "Mutation")]
impl PluginMutation {
    async fn stop(&self, context: &Context<'_>, name: String) -> Result<GraphQLPlugin> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        let plugin_resolver = context.data::<Arc<dyn PluginResolver + Send + Sync>>()?;
        let id = plugin_container_manager.get_id(&name).ok_or_else(|| Error::new("Plugin with name not found"))?;
        // Stop plugin
        plugin_container_manager
            .stop(&id)
            .map_err(|e| Error::new(format!("Failed to start {}: {:?}", &id, e)))?;
        // Make all transitions until the plugin and all dependent plugins have stopped
        plugin_resolver.resolve_until_idle().await;
        plugin_resolver.transition_to_fallback_states().await;
        Ok(GraphQLPlugin { id })
    }

    async fn start(&self, context: &Context<'_>, name: String) -> Result<GraphQLPlugin> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        let plugin_resolver = context.data::<Arc<dyn PluginResolver + Send + Sync>>()?;
        let id = plugin_container_manager.get_id(&name).ok_or_else(|| Error::new("Plugin with name not found"))?;
        // Start plugin
        plugin_container_manager
            .start(&id)
            .map_err(|e| Error::new(format!("Failed to start {}: {:?}", &id, e)))?;
        // Make all transitions until the plugin has started
        plugin_resolver.resolve_until_idle().await;
        // Start dependent plugins
        while plugin_container_manager.start_dependent_with_satisfied_dependencies(&id) {
            // Resolve until all dependent plugins are started
            plugin_resolver.resolve_until_idle().await;
        }
        plugin_resolver.transition_to_fallback_states().await;
        Ok(GraphQLPlugin { id })
    }

    async fn restart(&self, context: &Context<'_>, name: String) -> Result<GraphQLPlugin> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        let plugin_resolver = context.data::<Arc<dyn PluginResolver + Send + Sync>>()?;
        let id = plugin_container_manager.get_id(&name).ok_or_else(|| Error::new("Plugin with name not found"))?;
        // Stop plugin
        plugin_container_manager
            .stop(&id)
            .map_err(|e| Error::new(format!("Failed to start {}: {:?}", &id, e)))?;
        // Make all transitions until the plugin and all dependent plugins have stopped
        plugin_resolver.resolve_until_idle().await;
        // Start plugin
        plugin_container_manager
            .start(&id)
            .map_err(|e| Error::new(format!("Failed to start {}: {:?}", &id, e)))?;
        // Make all transitions until the plugin has started
        plugin_resolver.resolve_until_idle().await;
        // Start dependent plugins
        while plugin_container_manager.start_dependent_with_satisfied_dependencies(&id) {
            // Resolve until all dependent plugins are started
            plugin_resolver.resolve_until_idle().await;
        }
        plugin_resolver.transition_to_fallback_states().await;
        Ok(GraphQLPlugin { id })
    }

    /// Uninstalls a plugin
    async fn uninstall(&self, context: &Context<'_>, name: String) -> Result<bool> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        let plugin_resolver = context.data::<Arc<dyn PluginResolver + Send + Sync>>()?;
        let id = plugin_container_manager.get_id(&name).ok_or_else(|| Error::new("Plugin with name not found"))?;
        // plugin_container_manager.set_state(&id)
        plugin_container_manager
            .uninstall(&id)
            .map_err(|e| Error::new(format!("Failed to uninstall {}: {:?}", &id, e)))?;
        plugin_resolver.resolve_until_idle().await;
        plugin_resolver.transition_to_fallback_states().await;
        Ok(true)
    }

    /// Redeploys a plugin which is already installed, resolved or active.
    async fn redeploy(&self, context: &Context<'_>, name: String) -> Result<GraphQLPlugin> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        let plugin_resolver = context.data::<Arc<dyn PluginResolver + Send + Sync>>()?;
        let id = plugin_container_manager.get_id(&name).ok_or_else(|| Error::new("Plugin with name not found"))?;
        plugin_container_manager
            .redeploy(&id)
            .map_err(|e| Error::new(format!("Failed to start {}: {:?}", &id, e)))?;
        plugin_resolver.resolve_until_idle().await;
        plugin_resolver.transition_to_fallback_states().await;
        Ok(GraphQLPlugin { id })
    }
}
