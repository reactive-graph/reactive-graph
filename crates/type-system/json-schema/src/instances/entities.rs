use actix_web::get;
use actix_web::HttpResponse;
use schemars::schema_for;

use reactive_graph_graph::EntityInstance;

#[get("/instances/entities/schema")]
pub async fn schema_entity_instances() -> HttpResponse {
    let schema = schema_for!(EntityInstance);
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
