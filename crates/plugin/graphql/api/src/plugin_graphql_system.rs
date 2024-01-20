use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;

use crate::PluginQueryService;
use crate::PluginSchemaManager;

#[injectable]
#[async_trait]
pub trait PluginGraphQLSystem: Lifecycle {
    fn get_plugin_query_service(&self) -> Arc<dyn PluginQueryService + Send + Sync>;

    fn get_plugin_schema_manager(&self) -> Arc<dyn PluginSchemaManager + Send + Sync>;
}
