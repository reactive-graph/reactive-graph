use std::sync::Arc;

use async_graphql::Response;
use async_trait::async_trait;
use log::trace;
use springtime_di::component_alias;
use springtime_di::Component;

use inexor_rgf_dynamic_graph_api::DynamicGraphQueryService;
use inexor_rgf_dynamic_graph_api::DynamicGraphSchemaManager;
use inexor_rgf_dynamic_graph_api::DynamicQueryError;
use inexor_rgf_lifecycle::Lifecycle;

#[derive(Component)]
pub struct DynamicGraphQueryServiceImpl {
    dynamic_graph_schema_manager: Arc<dyn DynamicGraphSchemaManager + Send + Sync>,
}

impl DynamicGraphQueryServiceImpl {}

#[async_trait]
#[component_alias]
impl DynamicGraphQueryService for DynamicGraphQueryServiceImpl {
    async fn query(&self, request: String) -> Result<String, DynamicQueryError> {
        trace!("Run dynamic query: {}", request.clone());
        match self.dynamic_graph_schema_manager.get_dynamic_schema().await {
            Ok(schema) => {
                let result = schema.execute(request).await;
                serde_json::to_string(&result).map_err(DynamicQueryError::JsonError)
            }
            Err(e) => Err(DynamicQueryError::DynamicSchemaFailure(e)),
        }
    }

    async fn query_response(&self, request: &str) -> Result<Response, DynamicQueryError> {
        match self.dynamic_graph_schema_manager.get_dynamic_schema().await {
            Ok(schema) => Ok(schema.execute(request).await),
            Err(e) => Err(DynamicQueryError::DynamicSchemaFailure(e)),
        }
    }
}

#[async_trait]
impl Lifecycle for DynamicGraphQueryServiceImpl {
    async fn post_init(&self) {}
}
