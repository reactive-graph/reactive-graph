use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait PluginRepositoryManager: Send + Sync + Lifecycle {
    /// Scans the plugin hot deploy folder. Moves plugins to the plugin installation folder.
    fn scan_deploy_repository(&self);

    /// Scans the plugin installation folder and removes duplicates. If the same plugins exists
    /// multiple times, the plugin with the highest timestamp stays while all other are deleted.
    fn remove_duplicates(&self);

    /// Scans the plugin installation folder. Creates and registers new plugins to the
    /// plugin container manager.
    fn scan_plugin_repository(&self);

    /// Scans the plugin hot deploy folder.
    ///
    /// If a new plugin is detected it will be moved to the plugin installation folder
    /// and a new plugin container will be created and registered.
    ///
    /// If an existing plugin is detected a redeployment will be initiated.
    fn watch_hot_deploy(&self);

    fn unwatch_hot_deploy(&self);
}
