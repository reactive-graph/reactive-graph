use std::sync::Arc;

use crate::json_schema_response;
use actix_web::HttpResponse;
use actix_web::get;
use actix_web::web;
use mime::APPLICATION_JSON;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_json_schema::entities::schema_entity_types;

#[get("/types/entities")]
pub async fn get_entity_types(entity_type_manager: web::Data<Arc<dyn EntityTypeManager + Send + Sync>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .json(entity_type_manager.get_all())
}

#[get("/types/entities/{namespace:.*}")]
pub async fn get_entity_type(path: web::Path<String>, entity_type_manager: web::Data<Arc<dyn EntityTypeManager + Send + Sync>>) -> HttpResponse {
    let namespace = path.into_inner();
    let ty = match EntityTypeId::try_from(namespace) {
        Ok(ty) => ty,
        Err(e) => {
            return HttpResponse::NotFound().content_type(APPLICATION_JSON.to_string()).body(e.to_string());
        }
    };
    match entity_type_manager.get(&ty) {
        Some(entity_type) => HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(&entity_type),
        None => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Entity Type {ty} not found")),
    }
}

#[get("/types/entities/schema")]
pub async fn json_schema_entity_types() -> HttpResponse {
    json_schema_response(schema_entity_types())
}
