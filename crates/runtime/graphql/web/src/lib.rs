use actix_web::post;
use actix_web::web;
use async_graphql_actix_web::GraphQLRequest;
use async_graphql_actix_web::GraphQLResponse;

use inexor_rgf_runtime_graphql_schema::RuntimeSchema;

#[post("/runtime/graphql")]
pub async fn query_runtime_graphql(schema: web::Data<RuntimeSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}
