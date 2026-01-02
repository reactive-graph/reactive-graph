use std::str::FromStr;
use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::get;
use actix_web::web;
use mime::APPLICATION_JSON;

use crate::json_schema_response;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_json_schema::flows::schema_flow_types;

#[get("/types/flows")]
pub async fn get_flow_types(flow_type_manager: web::Data<Arc<dyn FlowTypeManager + Send + Sync>>) -> HttpResponse {
    HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(flow_type_manager.get_all())
}

#[get("/types/flows/{namespace:.*}")]
pub async fn get_flow_type(path: web::Path<String>, flow_type_manager: web::Data<Arc<dyn FlowTypeManager + Send + Sync>>) -> HttpResponse {
    let namespace = path.into_inner();
    let ty = match FlowTypeId::from_str(&namespace) {
        Ok(ty) => ty,
        Err(e) => {
            return HttpResponse::NotFound().content_type(APPLICATION_JSON.to_string()).body(e.to_string());
        }
    };
    match flow_type_manager.get(&ty) {
        Some(flow_type) => HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(&flow_type),
        None => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Flow Type {ty} not found")),
    }
}

#[get("/types/flows/schema")]
pub async fn json_schema_flow_types() -> HttpResponse {
    json_schema_response(schema_flow_types())
}
