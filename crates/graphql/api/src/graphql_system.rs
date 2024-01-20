use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;

use crate::GraphQLQueryService;
use crate::GraphQLSchemaManager;

#[injectable]
#[async_trait]
pub trait GraphQLSystem: Lifecycle {
    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService + Send + Sync>;

    fn get_graphql_schema_manager(&self) -> Arc<dyn GraphQLSchemaManager + Send + Sync>;
}
