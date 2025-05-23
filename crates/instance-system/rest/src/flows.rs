use actix_web::HttpResponse;
use actix_web::get;

use crate::json_schema_response;
use reactive_graph_instance_system_json_schema::flows::schema_flow_instances;

#[get("/instances/flows/schema")]
pub async fn json_schema_flow_instances() -> HttpResponse {
    json_schema_response(schema_flow_instances())
}
