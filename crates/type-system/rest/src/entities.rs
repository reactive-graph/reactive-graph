use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::get;
use actix_web::web;
use mime::APPLICATION_JSON;

use reactive_graph_type_system_api::EntityTypeManager;

#[get("/types/entities")]
pub async fn get_entity_types(entity_type_manager: web::Data<Arc<dyn EntityTypeManager + Send + Sync>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .json(entity_type_manager.get_all())
}

#[get("/types/entities/{namespace}/{type_name}")]
pub async fn get_entity_type(path: web::Path<(String, String)>, entity_type_manager: web::Data<Arc<dyn EntityTypeManager + Send + Sync>>) -> HttpResponse {
    let (namespace, type_name) = path.into_inner();
    match entity_type_manager.get_by_type(&namespace, &type_name) {
        Some(entity_type) => HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(&entity_type),
        None => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Entity Type {}__{} not found", namespace, type_name)),
    }
}
