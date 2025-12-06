use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_dynamic_graph_api::DynamicGraphQueryService;
use reactive_graph_dynamic_graph_api::DynamicGraphSchemaBuilder;
use reactive_graph_dynamic_graph_api::DynamicGraphSchemaManager;
use reactive_graph_dynamic_graph_api::DynamicGraphSystem;
use reactive_graph_dynamic_graph_api::SchemaBuilderContextManager;
use reactive_graph_dynamic_graph_api::SchemaBuilderManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_service_api::ReactiveSystem;
use reactive_graph_type_system_api::TypeSystemSystem;

#[derive(Component)]
pub struct DynamicGraphSystemImpl {
    dynamic_graph_query_service: Arc<dyn DynamicGraphQueryService + Send + Sync>,
    dynamic_graph_schema_builder: Arc<dyn DynamicGraphSchemaBuilder + Send + Sync>,
    dynamic_graph_schema_manager: Arc<dyn DynamicGraphSchemaManager + Send + Sync>,
    schema_builder_context_manager: Arc<dyn SchemaBuilderContextManager + Send + Sync>,
    schema_builder_manager: Arc<dyn SchemaBuilderManager + Send + Sync>,

    type_system_system: Arc<dyn TypeSystemSystem + Send + Sync>,
    reactive_system: Arc<dyn ReactiveSystem + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl DynamicGraphSystem for DynamicGraphSystemImpl {
    fn get_dynamic_graph_query_service(&self) -> Arc<dyn DynamicGraphQueryService + Send + Sync> {
        self.dynamic_graph_query_service.clone()
    }

    fn get_dynamic_graph_schema_builder(&self) -> Arc<dyn DynamicGraphSchemaBuilder + Send + Sync> {
        self.dynamic_graph_schema_builder.clone()
    }

    fn get_dynamic_graph_schema_manager(&self) -> Arc<dyn DynamicGraphSchemaManager + Send + Sync> {
        self.dynamic_graph_schema_manager.clone()
    }

    fn get_schema_builder_context_manager(&self) -> Arc<dyn SchemaBuilderContextManager + Send + Sync> {
        self.schema_builder_context_manager.clone()
    }

    fn get_schema_builder_manager(&self) -> Arc<dyn SchemaBuilderManager + Send + Sync> {
        self.schema_builder_manager.clone()
    }

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync> {
        self.type_system_system.clone()
    }

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync> {
        self.reactive_system.clone()
    }
}

#[async_trait]
impl Lifecycle for DynamicGraphSystemImpl {
    async fn init(&self) {
        self.dynamic_graph_schema_manager.init().await;
        self.dynamic_graph_query_service.init().await;
    }

    async fn post_init(&self) {
        self.dynamic_graph_schema_manager.post_init().await;
        self.dynamic_graph_query_service.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.dynamic_graph_query_service.pre_shutdown().await;
        self.dynamic_graph_schema_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.dynamic_graph_query_service.shutdown().await;
        self.dynamic_graph_schema_manager.shutdown().await;
    }
}
