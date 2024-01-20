use std::sync::Arc;

use actix_web::post;
use actix_web::web;
use async_graphql::dynamic::DynamicRequest;
use async_graphql::ServerError;
use async_graphql_actix_web::GraphQLRequest;
use async_graphql_actix_web::GraphQLResponse;

use inexor_rgf_dynamic_graph_api::DynamicGraphSchemaManager;

#[post("/dynamic_graph")]
pub async fn query_dynamic_graph(
    dynamic_graph_schema_manager: web::Data<Arc<dyn DynamicGraphSchemaManager + Send + Sync>>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    match dynamic_graph_schema_manager.get_dynamic_schema().await {
        Ok(schema) => {
            let dynamic_request = DynamicRequest::from(request.into_inner());
            schema.execute(dynamic_request).await.into()
        }
        Err(e) => {
            let errors = vec![ServerError::new(format!("Dynamic schema not available: {}", e), None)];
            async_graphql::Response::from_errors(errors).into()
        }
    }
}
