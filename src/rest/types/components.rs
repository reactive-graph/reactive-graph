use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use mime::APPLICATION_JSON;

use crate::api::ComponentManager;

#[get("/types/components")]
pub async fn get_components(component_manager: web::Data<Arc<dyn ComponentManager>>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON.to_string())
        .json(component_manager.get_components())
}
