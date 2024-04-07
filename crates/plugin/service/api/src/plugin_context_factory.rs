use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_api::PluginContext;

#[injectable]
#[async_trait]
pub trait PluginContextFactory: Send + Sync + Lifecycle {
    fn construct_plugin_context(&self);

    fn get(&self) -> Option<Arc<dyn PluginContext + Send + Sync>>;
}
