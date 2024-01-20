use actix_web::post;
use actix_web::web;
use async_graphql_actix_web::GraphQLRequest;
use async_graphql_actix_web::GraphQLResponse;

use inexor_rgf_plugin_graphql_schema::PluginSchema;

#[post("/plugin/graphql")]
pub async fn query_plugin_graphql(schema: web::Data<PluginSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}
