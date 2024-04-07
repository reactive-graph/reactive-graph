use actix_web::get;
use actix_web::HttpResponse;
use schemars::schema_for;

use reactive_graph_graph::FlowInstance;

#[get("/instances/flows/schema")]
pub async fn schema_flow_instances() -> HttpResponse {
    let schema = schema_for!(FlowInstance);
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
