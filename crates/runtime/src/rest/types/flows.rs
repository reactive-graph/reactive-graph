use std::sync::Arc;

use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use mime::APPLICATION_JSON;
use schemars::schema_for;

use crate::api::FlowTypeManager;
use crate::model::FlowTypeId;
use inexor_rgf_graph::FlowType;

#[get("/types/flows")]
pub async fn get_flow_types(flow_type_manager: web::Data<Arc<dyn FlowTypeManager>>) -> HttpResponse {
    HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(flow_type_manager.get_all())
}

#[get("/types/flows/{namespace}/{type_name}")]
pub async fn get_flow_type(
    namespace: web::Path<(String,)>,
    type_name: web::Path<(String,)>,
    flow_type_manager: web::Data<Arc<dyn FlowTypeManager>>,
) -> HttpResponse {
    let namespace = namespace.into_inner().0;
    let type_name = type_name.into_inner().0;
    let flow_ty = FlowTypeId::new_from_type(&namespace, &type_name);
    match flow_type_manager.get(&flow_ty) {
        Some(flow_type) => HttpResponse::Ok().content_type(APPLICATION_JSON.to_string()).json(&flow_type),
        None => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON.to_string())
            .body(format!("Flow Type {} not found", &flow_ty)),
    }
}

#[get("/types/flows/schema")]
pub async fn schema_flow_types() -> HttpResponse {
    let schema = schema_for!(FlowType);
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
