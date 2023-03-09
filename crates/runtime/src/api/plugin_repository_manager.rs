use async_trait::async_trait;

use crate::api::Lifecycle;

#[async_trait]
pub trait PluginRepositoryManager: Send + Sync + Lifecycle {
    /// Scans the folder plugins/deploy. Moves plugins to the folder plugins/installed.
    fn scan_deploy_repository(&self);

    /// Scans the folder plugins/installed. Creates and registers new plugins to the
    /// plugin container manager.
    fn scan_plugin_repository(&self);

    /// Scans the folder plugins/deploy.
    ///
    /// If a new plugin is detected it will be moved to the folder plugins/installed
    /// and a new plugin container will be created and registered.
    ///
    /// If an existing plugin is detected a redeployment will be initiated.
    fn watch_hot_deploy(&self);

    fn unwatch_hot_deploy(&self);
}
