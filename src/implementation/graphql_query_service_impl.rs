use async_trait::async_trait;
use log::info;

use crate::api::GraphQLQueryService;
use crate::api::GraphQLSchemaManager;
use crate::api::Lifecycle;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;
use async_std::task;

#[component]
pub struct GraphQLQueryServiceImpl {
    graphql_schema_manager: Wrc<dyn GraphQLSchemaManager>,
}

impl GraphQLQueryServiceImpl {}

#[async_trait]
#[provides]
impl GraphQLQueryService for GraphQLQueryServiceImpl {
    async fn query(&self, request: String) -> Result<String, serde_json::Error> {
        info!("Run query: {}", request.clone());
        let schema = self.graphql_schema_manager.get_schema();
        let result = schema.execute(request).await;
        serde_json::to_string(&result)
    }

    fn query_thread(&self, request: String) {
        let schema = self.graphql_schema_manager.get_schema();
        let _thread = task::Builder::new().name(String::from("query")).spawn(async move {
            info!("Run query in new thread: {}", request.clone());
            let result = schema.execute(request).await;
            let json = serde_json::to_string_pretty(&result);
            info!("query result: {}", json.unwrap());
        });
    }
}

impl Lifecycle for GraphQLQueryServiceImpl {
    fn post_init(&self) {
        let request = "query { types { countComponents countEntityTypes countRelationTypes countFlowTypes } instances { countEntityInstances countRelationInstances countFlowInstances } }";
        self.query_thread(request.to_string());
    }
}
