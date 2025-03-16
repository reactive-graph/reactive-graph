use actix_web::HttpResponse;
use actix_web::get;
use schemars::schema_for;

use reactive_graph_graph::EntityType;

#[get("/types/entities/schema")]
pub async fn schema_entity_types() -> HttpResponse {
    let schema = schema_for!(EntityType);
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
