use std::sync::Arc;

use crate::model::Component;
use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use mime::APPLICATION_JSON;
use schemars::schema_for;

use crate::api::ComponentManager;

#[get("/types/components")]
pub async fn get_components(component_manager: web::Data<Arc<dyn ComponentManager>>) -> HttpResponse {
    HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(component_manager.get_all())
}

#[get("/types/components/{namespace}/{type_name}")]
pub async fn get_component(
    namespace: web::Path<(String,)>,
    type_name: web::Path<(String,)>,
    component_manager: web::Data<Arc<dyn ComponentManager>>,
) -> HttpResponse {
    let namespace = namespace.into_inner().0;
    let type_name = type_name.into_inner().0;
    match component_manager.get_by_type(&namespace, &type_name) {
        Some(component) => HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(&component),
        None => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Component {}__{} not found", namespace, type_name)),
    }
}

#[get("/types/components/schema")]
pub async fn schema_components() -> HttpResponse {
    let schema = schema_for!(Component);
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
