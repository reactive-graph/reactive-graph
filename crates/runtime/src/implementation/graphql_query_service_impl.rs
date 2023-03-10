use async_trait::async_trait;
use log::info;

use crate::api::GraphQLQueryService;
use crate::api::GraphQLSchemaManager;
use crate::api::Lifecycle;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;

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
}

impl Lifecycle for GraphQLQueryServiceImpl {
    fn post_init(&self) {
    }
}
