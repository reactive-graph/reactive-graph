use async_trait::async_trait;
use log::trace;

use crate::api::DynamicGraphQueryService;
use crate::api::DynamicGraphSchemaManager;
use crate::api::DynamicQueryError;
use crate::api::Lifecycle;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;

#[component]
pub struct DynamicGraphQueryServiceImpl {
    dynamic_graph_schema_manager: Wrc<dyn DynamicGraphSchemaManager>,
}

impl DynamicGraphQueryServiceImpl {}

#[async_trait]
#[provides]
impl DynamicGraphQueryService for DynamicGraphQueryServiceImpl {
    async fn query(&self, request: String) -> Result<String, DynamicQueryError> {
        trace!("Run dynamic query: {}", request.clone());
        match self.dynamic_graph_schema_manager.get_dynamic_schema() {
            Some(schema) => {
                let result = schema.execute(request).await;
                serde_json::to_string(&result).map_err(|e| DynamicQueryError::JsonError(e))
            }
            None => Err(DynamicQueryError::DynamicSchemaFailure),
        }
    }
}

impl Lifecycle for DynamicGraphQueryServiceImpl {
    fn post_init(&self) {
    }
}