use std::sync::Arc;

use async_trait::async_trait;

use reactive_graph_plugin_api::Plugin;
use reactive_graph_plugin_api::PluginActivationError;
use reactive_graph_plugin_api::PluginDeactivationError;

/// A proxy object which wraps a [`Plugin`] and makes sure it can't outlive
/// the library it came from.
pub struct PluginProxy {
    pub(crate) plugin: Box<Arc<dyn Plugin>>,
}

#[async_trait]
impl Plugin for PluginProxy {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.plugin.activate().await
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.plugin.deactivate().await
    }
}
