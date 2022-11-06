use async_trait::async_trait;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::plugins::plugin_state::PluginStartError;
use crate::plugins::plugin_state::PluginStopError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PluginRegistryMode {
    Starting,
    Neutral,
    Stopping,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PluginTransitionResult {
    NoChange,
    Changed,
}

#[async_trait]
pub trait PluginRegistry: Send + Sync + Lifecycle {
    fn has(&self, stem: &str) -> bool;

    fn get_id(&self, stem: &str) -> Option<Uuid>;

    fn resolve_until_idle(&self);

    fn start(&self, id: &Uuid) -> Result<(), PluginStartError>;

    fn start_by_stem(&self, stem: &str) -> Result<(), PluginStartError>;

    fn stop(&self, id: &Uuid) -> Result<(), PluginStopError>;

    fn stop_by_stem(&self, stem: &str) -> Result<(), PluginStopError>;

    fn stop_all(&self);

    fn set_mode(&self, mode: PluginRegistryMode);

    fn get_mode(&self) -> PluginRegistryMode;
}
