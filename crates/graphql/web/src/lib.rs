use actix_web::post;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use async_graphql::Schema;
use async_graphql_actix_web::GraphQLRequest;
use async_graphql_actix_web::GraphQLResponse;
use async_graphql_actix_web::GraphQLSubscription;
use log::trace;
use reactive_graph_graphql_schema::ReactiveGraphSchema;

#[post("/graphql")]
pub async fn query_graphql(schema: web::Data<ReactiveGraphSchema>, request: GraphQLRequest) -> GraphQLResponse {
    let request = request.into_inner();
    trace!("{request:?}");
    let response = schema.execute(request).await;
    trace!("{response:?}");
    response.into()
}

pub async fn subscription_websocket(schema: web::Data<ReactiveGraphSchema>, request: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    // let mut data = Data::default();
    // if let Some(token) = get_token_from_headers(request.headers()) {
    //     data.insert(token);
    // }
    GraphQLSubscription::new(Schema::clone(&*schema))
        // .with_data(data)
        // .on_connection_init(on_connection_init)
        .start(&request, payload)
}
