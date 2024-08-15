use actix_web::post;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use async_graphql::Schema;
use async_graphql_actix_web::GraphQLRequest;
use async_graphql_actix_web::GraphQLResponse;
use async_graphql_actix_web::GraphQLSubscription;
use log::info;
use reactive_graph_graphql_schema::InexorSchema;

#[post("/graphql")]
pub async fn query_graphql(schema: web::Data<InexorSchema>, request: GraphQLRequest) -> GraphQLResponse {
    let request = request.into_inner();
    info!("{request:?}");
    let response = schema.execute(request).await;
    info!("{response:?}");
    let response = response.into();
    response
}

pub async fn subscription_websocket(schema: web::Data<InexorSchema>, request: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    // let mut data = Data::default();
    // if let Some(token) = get_token_from_headers(request.headers()) {
    //     data.insert(token);
    // }
    GraphQLSubscription::new(Schema::clone(&*schema))
        // .with_data(data)
        // .on_connection_init(on_connection_init)
        .start(&request, payload)
}
