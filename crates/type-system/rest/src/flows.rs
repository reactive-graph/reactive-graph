use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use mime::APPLICATION_JSON;

use reactive_graph_graph::FlowTypeId;
use reactive_graph_type_system_api::FlowTypeManager;

#[get("/types/flows")]
pub async fn get_flow_types(flow_type_manager: web::Data<Arc<dyn FlowTypeManager + Send + Sync>>) -> HttpResponse {
    HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(flow_type_manager.get_all())
}

#[get("/types/flows/{namespace}/{type_name}")]
pub async fn get_flow_type(path: web::Path<(String, String)>, flow_type_manager: web::Data<Arc<dyn FlowTypeManager + Send + Sync>>) -> HttpResponse {
    let (namespace, type_name) = path.into_inner();
    let flow_ty = FlowTypeId::new_from_type(namespace, type_name);
    match flow_type_manager.get(&flow_ty) {
        Some(flow_type) => HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(&flow_type),
        None => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Flow Type {} not found", &flow_ty)),
    }
}
