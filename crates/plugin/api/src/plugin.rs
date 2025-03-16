use async_trait::async_trait;

use crate::PluginActivationError;
use crate::PluginDeactivationError;
use crate::injectable;

pub const PLUGIN_NAME_PREFIX: &str = "reactive-graph-plugin-";

#[cfg_attr(feature = "springtime", injectable)]
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Called on initialization of the plugin.
    async fn activate(&self) -> Result<(), PluginActivationError> {
        Ok(())
    }

    /// Called on deactivation of the plugin.
    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        Ok(())
    }
}
