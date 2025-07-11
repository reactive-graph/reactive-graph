use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::get;
use actix_web::web;
use mime::APPLICATION_JSON;

use crate::json_schema_response;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_json_schema::components::schema_components;

#[get("/types/components")]
pub async fn get_components(component_manager: web::Data<Arc<dyn ComponentManager + Send + Sync>>) -> HttpResponse {
    HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(component_manager.get_all())
}

#[get("/types/components/{namespace}/{type_name}")]
pub async fn get_component(path: web::Path<(String, String)>, component_manager: web::Data<Arc<dyn ComponentManager + Send + Sync>>) -> HttpResponse {
    let (namespace, type_name) = path.into_inner();
    match component_manager.get_by_type(&namespace, &type_name) {
        Some(component) => HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(&component),
        None => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Component {namespace}__{type_name} not found")),
    }
}

#[get("/types/components/schema")]
pub async fn json_schema_components() -> HttpResponse {
    json_schema_response(schema_components())
}
