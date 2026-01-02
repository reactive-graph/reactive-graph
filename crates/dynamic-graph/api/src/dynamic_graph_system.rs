use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_service_api::ReactiveSystem;
use reactive_graph_type_system_api::TypeSystemSystem;

use crate::DynamicGraphQueryService;
use crate::DynamicGraphSchemaBuilder;
use crate::DynamicGraphSchemaManager;
use crate::SchemaBuilderContextManager;
use crate::SchemaBuilderManager;

#[injectable]
#[async_trait]
pub trait DynamicGraphSystem: Lifecycle {
    fn get_dynamic_graph_query_service(&self) -> Arc<dyn DynamicGraphQueryService + Send + Sync>;

    fn get_dynamic_graph_schema_builder(&self) -> Arc<dyn DynamicGraphSchemaBuilder + Send + Sync>;

    fn get_dynamic_graph_schema_manager(&self) -> Arc<dyn DynamicGraphSchemaManager + Send + Sync>;

    fn get_schema_builder_context_manager(&self) -> Arc<dyn SchemaBuilderContextManager + Send + Sync>;

    fn get_schema_builder_manager(&self) -> Arc<dyn SchemaBuilderManager + Send + Sync>;

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync>;

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync>;
}
