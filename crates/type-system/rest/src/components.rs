use std::sync::Arc;

use crate::json_schema_response;
use actix_web::HttpResponse;
use actix_web::get;
use actix_web::web;
use mime::APPLICATION_JSON;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_json_schema::components::schema_components;

#[get("/types/components")]
pub async fn get_components(component_manager: web::Data<Arc<dyn ComponentManager + Send + Sync>>) -> HttpResponse {
    HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(component_manager.get_all())
}

#[get("/types/components/{namespace:.*}")]
pub async fn get_component(path: web::Path<String>, component_manager: web::Data<Arc<dyn ComponentManager + Send + Sync>>) -> HttpResponse {
    let namespace = path.into_inner();
    let ty = match ComponentTypeId::try_from(namespace) {
        Ok(ty) => ty,
        Err(e) => {
            return HttpResponse::NotFound().content_type(APPLICATION_JSON.to_string()).body(e.to_string());
        }
    };
    match component_manager.get(&ty) {
        Some(component) => HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(&component),
        None => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Component {ty} not found")),
    }
}

#[get("/types/components/schema")]
pub async fn json_schema_components() -> HttpResponse {
    json_schema_response(schema_components())
}
