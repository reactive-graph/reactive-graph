use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use mime::APPLICATION_JSON;

use crate::api::RelationTypeManager;

#[get("/types/relations")]
pub async fn get_relation_types(relation_type_manager: web::Data<Arc<dyn RelationTypeManager>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .json(relation_type_manager.get_relation_types())
}

#[get("/types/relations/{name}")]
pub async fn get_relation_type(name: web::Path<(String,)>, relation_type_manager: web::Data<Arc<dyn RelationTypeManager>>) -> HttpResponse {
    let name = name.into_inner().0;
    let relation_type = relation_type_manager.get(name.clone());
    if relation_type.is_some() {
        HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(relation_type)
    } else {
        HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Relation Type {} not found", name))
    }
}
