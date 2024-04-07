use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;

use crate::RuntimeQueryService;
use crate::RuntimeSchemaManager;

#[injectable]
#[async_trait]
pub trait RuntimeGraphQLSystem: Lifecycle {
    fn get_runtime_query_service(&self) -> Arc<dyn RuntimeQueryService + Send + Sync>;

    fn get_runtime_schema_manager(&self) -> Arc<dyn RuntimeSchemaManager + Send + Sync>;
}
