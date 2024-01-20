use actix_web::get;
use actix_web::HttpResponse;
use schemars::schema_for;

use inexor_rgf_graph::RelationInstance;

#[get("/instances/relations/schema")]
pub async fn schema_relation_instances() -> HttpResponse {
    let schema = schema_for!(RelationInstance);
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
