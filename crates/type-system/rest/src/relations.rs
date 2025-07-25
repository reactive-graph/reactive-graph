use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::get;
use actix_web::web;
use mime::APPLICATION_JSON;

use crate::json_schema_response;
use reactive_graph_type_system_api::RelationTypeManager;
use reactive_graph_type_system_json_schema::relations::schema_relation_types;

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
            .body(format!("Relation Type {namespace}__{type_name} not found")),
    }
}

#[get("/types/relations/schema")]
pub async fn json_schema_relation_types() -> HttpResponse {
    json_schema_response(schema_relation_types())
}
