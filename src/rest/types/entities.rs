use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use mime::APPLICATION_JSON;

use crate::api::EntityTypeManager;

#[get("/types/entities")]
pub async fn get_entity_types(entity_type_manager: web::Data<Arc<dyn EntityTypeManager>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .json(entity_type_manager.get_entity_types())
}

#[get("/types/entities/{name}")]
pub async fn get_entity_type(name: web::Path<(String,)>, entity_type_manager: web::Data<Arc<dyn EntityTypeManager>>) -> HttpResponse {
    let name = name.into_inner().0;
    let entity_type = entity_type_manager.get(name.clone());
    if entity_type.is_some() {
        HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(entity_type)
    } else {
        HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Entity Type {} not found", name))
    }
}
