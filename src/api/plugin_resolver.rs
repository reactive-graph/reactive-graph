use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::plugin::PluginResolverMode;
use crate::plugin::PluginTransitionResult;

#[async_trait]
pub trait PluginResolver: Send + Sync + Lifecycle {
    fn resolve_until_idle(&self);

    fn resolve(&self) -> PluginTransitionResult;

    fn set_mode(&self, mode: PluginResolverMode);

    fn get_mode(&self) -> PluginResolverMode;
}
