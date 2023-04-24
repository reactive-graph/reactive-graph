use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use actix_cors::Cors;
use actix_http::body::BoxBody;
use actix_web::dev::Server;
use actix_web::guard;
use actix_web::post;
use actix_web::web;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use actix_web::HttpServer;
use actix_web::Result;
use actix_web_extras::middleware::Condition;
use async_graphql::dynamic::DynamicRequest;
use async_graphql::Schema;
use async_graphql::ServerError;
use async_graphql_actix_web::GraphQLRequest;
use async_graphql_actix_web::GraphQLResponse;
use async_graphql_actix_web::GraphQLSubscription;
use async_trait::async_trait;
use crossbeam::channel::Receiver;
use http::header::CONTENT_TYPE;
use http::Request;
use http::Response;
use log::debug;
use log::error;
use log::info;
use log::trace;
use rustls::Certificate;
use rustls::PrivateKey;
use rustls::ServerConfig;
use rustls_pemfile::certs;
use rustls_pemfile::pkcs8_private_keys;
use serde::Deserialize;

use crate::api::ComponentManager;
use crate::api::ConfigManager;
use crate::api::DynamicGraphSchemaManager;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::GraphQLSchemaManager;
use crate::api::GraphQLServer;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveFlowInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationTypeManager;
use crate::api::TypeCategoryManager;
use crate::api::WebResourceManager;
use crate::di::*;
use crate::graphql::InexorSchema;
use crate::implementation::get_logger_middleware;
use crate::plugins::HttpBody;

#[component]
pub struct GraphQLServerImpl {
    component_manager: Wrc<dyn ComponentManager>,

    config_manager: Wrc<dyn ConfigManager>,

    dynamic_graph_schema_manager: Wrc<dyn DynamicGraphSchemaManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    flow_type_manager: Wrc<dyn FlowTypeManager>,

    type_category_manager: Wrc<dyn TypeCategoryManager>,

    entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,

    flow_instance_manager: Wrc<dyn ReactiveFlowInstanceManager>,

    graphql_schema_manager: Wrc<dyn GraphQLSchemaManager>,

    web_resource_manager: Wrc<dyn WebResourceManager>,
}

#[post("/graphql")]
async fn query_graphql(schema: web::Data<InexorSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

#[post("/dynamic_graph")]
async fn query_dynamic_graph(dynamic_graph_schema_manager: web::Data<Arc<dyn DynamicGraphSchemaManager>>, request: GraphQLRequest) -> GraphQLResponse {
    match dynamic_graph_schema_manager.get_dynamic_schema().await {
        Some(schema) => {
            let dynamic_request = DynamicRequest::from(request.into_inner());
            schema.execute(dynamic_request).await.into()
        }
        None => async_graphql::Response::from_errors(vec![ServerError::new("Dynamic schema not available", None)]).into(),
    }
}

async fn subscription_websocket(schema: web::Data<InexorSchema>, request: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    // let mut data = Data::default();
    // if let Some(token) = get_token_from_headers(request.headers()) {
    //     data.insert(token);
    // }
    GraphQLSubscription::new(Schema::clone(&*schema))
        // .with_data(data)
        // .on_connection_init(on_connection_init)
        .start(&request, payload)
}

#[derive(Deserialize)]
pub struct WebResourcePathInfo {
    web_resource_context_path: String,
    path: String,
}

pub async fn handle_web_resource(
    web_resource_manager: web::Data<Arc<dyn WebResourceManager>>,
    path_info: web::Path<WebResourcePathInfo>,
    request: HttpRequest,
) -> HttpResponse {
    let context_path = path_info.web_resource_context_path.clone();
    let path = path_info.path.clone();
    let uri = request.uri().clone();
    let http_request = convert_request(request);
    match web_resource_manager.get(context_path.clone()) {
        Some(web_resource) => match web_resource.handle_web_resource(path, http_request) {
            Ok(response) => convert_response(response),
            Err(err) => HttpResponse::InternalServerError().body(format!("500 Internal Server Error: {}", err)),
        },
        None => match web_resource_manager.get_default() {
            Some(web_resource) => match web_resource.handle_web_resource(format!("{}/{}", context_path, path), http_request) {
                Ok(response) => convert_response(response),
                Err(err) => HttpResponse::InternalServerError().body(format!("500 Internal Server Error: {}", err)),
            },
            None => HttpResponse::NotFound().body(format!("404 Not Found: {}", uri)),
        },
    }
}

#[derive(Deserialize)]
pub struct RootPathInfo {
    path: String,
}

pub async fn handle_root_web_resource(
    web_resource_manager: web::Data<Arc<dyn WebResourceManager>>,
    path_info: web::Path<RootPathInfo>,
    request: HttpRequest,
) -> HttpResponse {
    let path = path_info.path.clone();
    let uri = request.uri().clone();
    debug!("path: {} uri: {}", path, uri);
    let http_request = convert_request(request);
    match web_resource_manager.get_default() {
        Some(web_resource) => match web_resource.handle_web_resource(path, http_request) {
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

#[async_trait]
#[provides]
impl GraphQLServer for GraphQLServerImpl {
    async fn serve(&self, stopper: Receiver<()>) {
        trace!("Initialize GraphQL server");
        let server = match self.setup() {
            Ok(server) => server,
            Err(error) => {
                error!("Failed to setup graphql server: {}!", error);
                return;
            }
        };
        let server_handle = server.handle();
        let t_server_handle = server_handle.clone();

        let terminate = Arc::new(AtomicBool::new(false));
        let t_terminate = terminate.clone();

        // This thread handles the server stop routine from the main thread
        tokio::spawn(async move {
            trace!("Waiting for shutdown signal");
            // wait for shutdown signal
            stopper.recv().unwrap_or(());
            debug!("Received shutdown signal. Stopping GraphQL server thread.");

            // stop server gracefully
            trace!("Stopping server gracefully");
            t_server_handle.stop(true).await;
            debug!("Successfully stopped GraphQL server thread.");

            t_terminate.store(true, Ordering::Relaxed);
            debug!("Stopping actix system.");
        });

        let _ = server.await;
        trace!("GraphQL server finished");
    }
}

impl GraphQLServerImpl {
    fn setup(&self) -> Result<Server> {
        let graphql_server_config = self.config_manager.get_graphql_server_config();
        let graphql_logging_config = graphql_server_config.logging.as_ref().cloned().unwrap_or_default();

        // GraphQL Schema
        let schema = self.graphql_schema_manager.get_schema();

        // REST SERVICES
        let component_manager = web::Data::new(self.component_manager.clone());
        let entity_type_manager = web::Data::new(self.entity_type_manager.clone());
        let relation_type_manager = web::Data::new(self.relation_type_manager.clone());
        let flow_type_manager = web::Data::new(self.flow_type_manager.clone());
        let type_category_manager = web::Data::new(self.type_category_manager.clone());
        let entity_instance_manager = web::Data::new(self.entity_instance_manager.clone());
        let relation_instance_manager = web::Data::new(self.relation_instance_manager.clone());
        let flow_instance_manager = web::Data::new(self.flow_instance_manager.clone());
        let web_resource_manager = web::Data::new(self.web_resource_manager.clone());
        let schema_data = web::Data::new(schema);
        let dynamic_graph_schema_manager = web::Data::new(self.dynamic_graph_schema_manager.clone());

        let http_server = HttpServer::new(move || {
            let graphql_logging_config = graphql_logging_config.clone();
            App::new()
                .wrap(Cors::permissive())
                .wrap(Condition::from_option(get_logger_middleware(&graphql_logging_config)))
                .app_data(schema_data.clone())
                .app_data(component_manager.clone())
                .app_data(entity_type_manager.clone())
                .app_data(relation_type_manager.clone())
                .app_data(flow_type_manager.clone())
                .app_data(type_category_manager.clone())
                .app_data(entity_instance_manager.clone())
                .app_data(relation_instance_manager.clone())
                .app_data(flow_instance_manager.clone())
                .app_data(web_resource_manager.clone())
                .app_data(dynamic_graph_schema_manager.clone())
                // GraphQL API
                .service(query_graphql)
                .service(
                    web::resource("/graphql")
                        .guard(guard::Get())
                        .guard(guard::Header("upgrade", "websocket"))
                        .to(subscription_websocket),
                )
                // Dynamic GraphQL API
                .service(query_dynamic_graph)
                // REST API
                .service(crate::rest::types::components::get_components)
                .service(crate::rest::types::entities::get_entity_types)
                .service(crate::rest::types::entities::get_entity_type)
                .service(crate::rest::types::relations::get_relation_types)
                .service(crate::rest::types::relations::get_relation_type)
                // Web Resource API
                .service(web::resource("/{web_resource_context_path}/{path:.*}").route(web::get().to(handle_web_resource)))
                .service(web::resource("/{path:.*}").route(web::get().to(handle_root_web_resource)))
        })
        .disable_signals()
        .shutdown_timeout(graphql_server_config.shutdown_timeout())
        .workers(graphql_server_config.workers());

        let r_http_server = if graphql_server_config.is_secure() {
            let cert_file = &mut BufReader::new(File::open("./keys/cert.pem").unwrap());
            let key_file = &mut BufReader::new(File::open("./keys/key.pem").unwrap());
            let cert_chain = certs(cert_file).unwrap().into_iter().map(Certificate).collect();
            let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file).unwrap().into_iter().map(PrivateKey).collect();
            if keys.is_empty() {
                error!("Could not locate PKCS 8 private keys.");
            }
            let tls_config = ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(cert_chain, keys.remove(0))
                .unwrap();
            info!("Starting HTTP/GraphQL server on {}", graphql_server_config.url());
            http_server.bind_rustls(graphql_server_config.addr(), tls_config)?.run()
        } else {
            info!("Starting HTTP/GraphQL server on {}", graphql_server_config.url());
            http_server.bind(graphql_server_config.addr())?.run()
        };
        Ok(r_http_server)
    }
}

#[async_trait]
impl Lifecycle for GraphQLServerImpl {
    async fn init(&self) {}

    async fn post_init(&self) {}

    async fn pre_shutdown(&self) {}

    async fn shutdown(&self) {}
}
