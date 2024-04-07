use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_runtime_graphql_api::RuntimeGraphQLSystem;
use reactive_graph_runtime_graphql_api::RuntimeQueryService;
use reactive_graph_runtime_graphql_api::RuntimeSchemaManager;

#[derive(Component)]
pub struct RuntimeGraphQLSystemImpl {
    runtime_query_service: Arc<dyn RuntimeQueryService + Send + Sync>,
    runtime_schema_manager: Arc<dyn RuntimeSchemaManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RuntimeGraphQLSystem for RuntimeGraphQLSystemImpl {
    fn get_runtime_query_service(&self) -> Arc<dyn RuntimeQueryService + Send + Sync> {
        self.runtime_query_service.clone()
    }

    fn get_runtime_schema_manager(&self) -> Arc<dyn RuntimeSchemaManager + Send + Sync> {
        self.runtime_schema_manager.clone()
    }
}

#[async_trait]
impl Lifecycle for RuntimeGraphQLSystemImpl {
    async fn init(&self) {
        self.runtime_schema_manager.init().await;
        self.runtime_query_service.init().await;
    }

    async fn post_init(&self) {
        self.runtime_schema_manager.post_init().await;
        self.runtime_query_service.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.runtime_query_service.pre_shutdown().await;
        self.runtime_schema_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.runtime_query_service.shutdown().await;
        self.runtime_schema_manager.shutdown().await;
    }
}
