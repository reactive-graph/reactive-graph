use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use mime::APPLICATION_JSON;
use schemars::schema_for;

use crate::api::EntityTypeManager;
use crate::model::EntityType;

#[get("/types/entities")]
pub async fn get_entity_types(entity_type_manager: web::Data<Arc<dyn EntityTypeManager>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .json(entity_type_manager.get_all())
}

#[get("/types/entities/{namespace}/{type_name}")]
pub async fn get_entity_type(
    namespace: web::Path<(String,)>,
    type_name: web::Path<(String,)>,
    entity_type_manager: web::Data<Arc<dyn EntityTypeManager>>,
) -> HttpResponse {
    let namespace = namespace.into_inner().0;
    let type_name = type_name.into_inner().0;
    match entity_type_manager.get_by_type(&namespace, &type_name) {
        Some(entity_type) => HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(&entity_type),
        None => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Entity Type {}__{} not found", namespace, type_name)),
    }
}

#[get("/types/entities/schema")]
pub async fn schema_entity_types() -> HttpResponse {
    let schema = schema_for!(EntityType);
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
