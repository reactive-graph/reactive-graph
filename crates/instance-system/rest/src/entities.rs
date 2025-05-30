use actix_web::HttpResponse;
use actix_web::get;

use crate::json_schema_response;
use reactive_graph_instance_system_json_schema::schema_entity_instances;

#[get("/instances/entities/schema")]
pub async fn json_schema_entity_instances() -> HttpResponse {
    json_schema_response(schema_entity_instances())
}
