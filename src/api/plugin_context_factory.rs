use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::plugins::PluginContext;

#[async_trait]
pub trait PluginContextFactory: Send + Sync + Lifecycle {
    fn construct_plugin_context(&self);

    fn get(&self) -> Option<Arc<dyn PluginContext>>;
}
