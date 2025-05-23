use actix_web::HttpResponse;
use actix_web::get;

use crate::json_schema_response;
use reactive_graph_instance_system_json_schema::relations::schema_relation_instances;

#[get("/instances/relations/schema")]
pub async fn json_schema_relation_instances() -> HttpResponse {
    json_schema_response(schema_relation_instances())
}
