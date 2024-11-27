use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use mime::APPLICATION_JSON;

use reactive_graph_type_system_api::RelationTypeManager;

#[get("/types/relations")]
pub async fn get_relation_types(relation_type_manager: web::Data<Arc<dyn RelationTypeManager + Send + Sync>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .json(relation_type_manager.get_all())
}

#[get("/types/relations/{namespace}/{type_name}")]
pub async fn get_relation_type(
    path: web::Path<(String, String)>,
    relation_type_manager: web::Data<Arc<dyn RelationTypeManager + Send + Sync>>,
) -> HttpResponse {
    let (namespace, type_name) = path.into_inner();
    match relation_type_manager.get_by_type(&namespace, &type_name) {
        Some(relation_type) => HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(&relation_type),
        None => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Relation Type {}__{} not found", namespace, type_name)),
    }
}
