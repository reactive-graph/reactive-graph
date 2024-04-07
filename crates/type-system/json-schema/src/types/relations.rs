use actix_web::get;
use actix_web::HttpResponse;
use schemars::schema_for;

use reactive_graph_graph::RelationType;

#[get("/types/relations/schema")]
pub async fn schema_relation_types() -> HttpResponse {
    let schema = schema_for!(RelationType);
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
