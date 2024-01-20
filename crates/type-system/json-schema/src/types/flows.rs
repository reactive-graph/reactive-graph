use actix_web::get;
use actix_web::HttpResponse;
use schemars::schema_for;

use inexor_rgf_graph::FlowType;

#[get("/types/flows/schema")]
pub async fn schema_flow_types() -> HttpResponse {
    let schema = schema_for!(FlowType);
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
