use actix_web::get;
use actix_web::HttpResponse;
use schemars::schema_for;

use reactive_graph_graph::Component;

#[get("/types/components/schema")]
pub async fn schema_components() -> HttpResponse {
    let schema = schema_for!(Component);
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
