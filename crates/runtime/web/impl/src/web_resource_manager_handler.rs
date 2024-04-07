use std::sync::Arc;

use actix_http::body::BoxBody;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use http::header::CONTENT_TYPE;
use http::Request;
use http::Response;
use log::debug;

use reactive_graph_plugin_api::HttpBody;
use reactive_graph_runtime_web_api::RootPathInfo;
use reactive_graph_runtime_web_api::WebResourceManager;
use reactive_graph_runtime_web_api::WebResourcePathInfo;

pub async fn handle_web_resource(
    web_resource_manager: web::Data<Arc<dyn WebResourceManager + Send + Sync>>,
    path_info: web::Path<WebResourcePathInfo>,
    request: HttpRequest,
) -> HttpResponse {
    let context_path = path_info.web_resource_context_path.clone();
    let path = path_info.path.clone();
    let uri = request.uri().clone();
    let http_request = convert_request(request);
    match web_resource_manager.get(context_path.clone()) {
        Some(web_resource) => match web_resource.handle_web_resource(path, http_request).await {
            Ok(response) => convert_response(response),
            Err(err) => HttpResponse::InternalServerError().body(format!("500 Internal Server Error: {}", err)),
        },
        None => match web_resource_manager.get_default() {
            Some(web_resource) => match web_resource.handle_web_resource(format!("{}/{}", context_path, path), http_request).await {
                Ok(response) => convert_response(response),
                Err(err) => HttpResponse::InternalServerError().body(format!("500 Internal Server Error: {}", err)),
            },
            None => HttpResponse::NotFound().body(format!("404 Not Found: {}", uri)),
        },
    }
}

pub async fn handle_root_web_resource(
    web_resource_manager: web::Data<Arc<dyn WebResourceManager + Send + Sync>>,
    path_info: web::Path<RootPathInfo>,
    request: HttpRequest,
) -> HttpResponse {
    let path = path_info.path.clone();
    let uri = request.uri().clone();
    debug!("path: {} uri: {}", path, uri);
    let http_request = convert_request(request);
    match web_resource_manager.get_default() {
        Some(web_resource) => match web_resource.handle_web_resource(path, http_request).await {
            Ok(response) => convert_response(response),
            Err(err) => HttpResponse::InternalServerError().body(format!("500 Internal Server Error: {}", err)),
        },
        None => HttpResponse::NotFound().body(format!("404 Not Found: {}", uri)),
    }
}

fn convert_request(request: HttpRequest) -> Request<HttpBody> {
    let mut request_builder = http::request::Builder::new()
        .uri(request.uri())
        .method(request.method())
        .version(request.version());
    if let Some(headers_map) = request_builder.headers_mut() {
        request.headers().into_iter().for_each(|(header_name, header_value)| {
            headers_map.insert(header_name, header_value.clone());
        });
    }
    request_builder.body(HttpBody::None).unwrap()
}

fn convert_response(response: Response<HttpBody>) -> HttpResponse {
    let mut response_builder = HttpResponseBuilder::new(response.status());
    if let Some(header) = response.headers().get(CONTENT_TYPE) {
        response_builder.content_type(header);
    }
    response.headers().into_iter().for_each(|header| {
        response_builder.append_header(header);
    });
    response_builder.body(match response.into_body() {
        HttpBody::None => BoxBody::new(()),
        HttpBody::Binary(bytes) => BoxBody::new(bytes),
        HttpBody::Json(value) => BoxBody::new(serde_json::to_string(&value).unwrap_or_default()),
        HttpBody::PlainText(content) => BoxBody::new(content),
    })
}
