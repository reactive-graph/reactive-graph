use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::plugin::PluginResolverMode;
use crate::plugin::PluginTransitionResult;

#[async_trait]
pub trait PluginResolver: Send + Sync + Lifecycle {
    /// Resolves plugins until no more resolve action is possible.
    async fn resolve_until_idle(&self);

    /// Stops all plugins until all are stopped.
    async fn stop_until_all_stopped(&self);

    /// Runs the next resolve action.
    async fn resolve(&self) -> PluginTransitionResult;

    async fn transition_to_fallback_states(&self);

    /// Sets the resolve mode.
    fn set_mode(&self, mode: PluginResolverMode);

    /// Returns the resolve mode.
    fn get_mode(&self) -> PluginResolverMode;
}
