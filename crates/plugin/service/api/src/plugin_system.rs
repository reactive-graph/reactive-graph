use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use crate::PluginContainerManager;
use crate::PluginContextFactory;
use crate::PluginRepositoryManager;
use crate::PluginResolver;
use inexor_rgf_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait PluginSystem: Lifecycle {
    fn get_plugin_context_factory(&self) -> Arc<dyn PluginContextFactory + Send + Sync>;

    fn get_plugin_container_manager(&self) -> Arc<dyn PluginContainerManager + Send + Sync>;

    fn get_plugin_repository_manager(&self) -> Arc<dyn PluginRepositoryManager + Send + Sync>;

    fn get_plugin_resolver(&self) -> Arc<dyn PluginResolver + Send + Sync>;
}
