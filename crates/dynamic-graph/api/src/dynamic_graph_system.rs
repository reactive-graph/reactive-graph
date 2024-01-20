use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_service_api::ReactiveSystem;
use inexor_rgf_type_system_api::TypeSystem;

use crate::DynamicGraphQueryService;
use crate::DynamicGraphSchemaManager;

#[injectable]
#[async_trait]
pub trait DynamicGraphSystem: Lifecycle {
    fn get_dynamic_graph_query_service(&self) -> Arc<dyn DynamicGraphQueryService + Send + Sync>;

    fn get_dynamic_graph_schema_manager(&self) -> Arc<dyn DynamicGraphSchemaManager + Send + Sync>;

    fn type_system(&self) -> Arc<dyn TypeSystem + Send + Sync>;

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync>;
}
