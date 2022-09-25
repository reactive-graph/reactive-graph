use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::plugin::proxy::PluginProxy;
use crate::plugins::PluginInitializationError;
use crate::plugins::PluginLoadingError;
use crate::plugins::PluginPostInitializationError;
use crate::plugins::PluginPreShutdownError;
use crate::plugins::PluginShutdownError;

#[async_trait]
pub trait PluginRegistry: Send + Sync + Lifecycle {
    fn has(&self, name: String) -> bool;

    fn get(&self, name: String) -> Option<Arc<PluginProxy>>;

    fn load_plugins(&self);

    fn load_plugin(&self, name: String, path: String);

    fn unload_plugins(&self);

    unsafe fn load(&self, library_path: String) -> Result<(), PluginLoadingError>;
    // unsafe fn unload<P: AsRef<OsStr>>(&mut self, library_path: P) -> Result<(), PluginError>;

    fn plugin_init(&self, name: String) -> Result<(), PluginInitializationError>;

    fn plugin_post_init(&self, name: String) -> Result<(), PluginPostInitializationError>;

    fn plugin_pre_shutdown(&self, name: String) -> Result<(), PluginPreShutdownError>;

    fn plugin_shutdown(&self, name: String) -> Result<(), PluginShutdownError>;
}
