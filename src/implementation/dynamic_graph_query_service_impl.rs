use async_trait::async_trait;
use log::error;
use log::trace;

use crate::api::DynamicGraphQueryService;
use crate::api::DynamicGraphSchemaManager;
use crate::api::DynamicQueryError;
use crate::api::Lifecycle;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;
use async_std::task;

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

    fn query_thread(&self, request: String) {
        let dynamic_graph_schema_manager = self.dynamic_graph_schema_manager.clone();
        let _thread = task::Builder::new().name(String::from("dynamic query")).spawn(async move {
            match dynamic_graph_schema_manager.get_dynamic_schema() {
                Some(schema) => {
                    trace!("Run dynamic query in new thread: {}", request.clone());
                    let result = schema.execute(request).await;
                    match serde_json::to_string_pretty(&result) {
                        Ok(json) => trace!("Dynamic query result:\n{}", json),
                        Err(e) => error!("Failed to execute dynamic query: {}", e),
                    }
                }
                None => {
                    error!("Failed to get dynamic schema");
                }
            }
        });
    }
}

impl Lifecycle for DynamicGraphQueryServiceImpl {
    fn post_init(&self) {
        let request = "query { core { shutdown { id label } }}";
        self.query_thread(request.to_string());
    }
}
