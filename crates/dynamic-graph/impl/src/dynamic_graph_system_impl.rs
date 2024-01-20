use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use inexor_rgf_dynamic_graph_api::DynamicGraphQueryService;
use inexor_rgf_dynamic_graph_api::DynamicGraphSchemaManager;
use inexor_rgf_dynamic_graph_api::DynamicGraphSystem;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_service_api::ReactiveSystem;
use inexor_rgf_type_system_api::TypeSystem;

#[derive(Component)]
pub struct DynamicGraphSystemImpl {
    dynamic_graph_query_service: Arc<dyn DynamicGraphQueryService + Send + Sync>,
    dynamic_graph_schema_manager: Arc<dyn DynamicGraphSchemaManager + Send + Sync>,

    type_system: Arc<dyn TypeSystem + Send + Sync>,
    reactive_system: Arc<dyn ReactiveSystem + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl DynamicGraphSystem for DynamicGraphSystemImpl {
    fn get_dynamic_graph_query_service(&self) -> Arc<dyn DynamicGraphQueryService + Send + Sync> {
        self.dynamic_graph_query_service.clone()
    }

    fn get_dynamic_graph_schema_manager(&self) -> Arc<dyn DynamicGraphSchemaManager + Send + Sync> {
        self.dynamic_graph_schema_manager.clone()
    }

    fn type_system(&self) -> Arc<dyn TypeSystem + Send + Sync> {
        self.type_system.clone()
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
