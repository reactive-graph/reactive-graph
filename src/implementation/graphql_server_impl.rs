use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::di::*;
use actix_cors::Cors;
use actix_http::body::BoxBody;
use actix_web::{guard, post, web, App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer, Result};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use async_std::task;
use async_trait::async_trait;
use http::header::CONTENT_TYPE;
use http::{Request, Response};
use inexor_rgf_core_plugins::HttpBody;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

use crate::api::{
    ComponentManager, EntityTypeManager, GraphQLServer, Lifecycle, ReactiveEntityInstanceManager, ReactiveFlowManager, ReactiveRelationInstanceManager,
    RelationTypeManager, WebResourceManager,
};
use crate::graphql::{InexorMutation, InexorQuery, InexorSchema, InexorSubscription};

#[component]
pub struct GraphQLServerImpl {
    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,

    flow_manager: Wrc<dyn ReactiveFlowManager>,

    web_resource_manager: Wrc<dyn WebResourceManager>,
}

#[post("/graphql")]
async fn query_graphql(schema: web::Data<InexorSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
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
    web_resource_base_path: String,
    path: String,
}

pub async fn handle_web_resource(
    web_resource_manager: web::Data<Arc<dyn WebResourceManager>>,
    path_info: web::Path<WebResourcePathInfo>,
    request: HttpRequest,
) -> HttpResponse {
    let base_path = path_info.web_resource_base_path.clone();
    let path = path_info.path.clone();
    let uri = request.uri().clone();
    debug!("base_path = {}", base_path.as_str());
    debug!("path = {}", path.as_str());
    let http_request = convert_request(request);
    match web_resource_manager.get(base_path.clone()) {
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
    let http_request = request_builder.body(HttpBody::None).unwrap();
    http_request
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
        HttpBody::Json(value) => BoxBody::new(serde_json::to_string(&value).unwrap_or(String::default())),
        HttpBody::PlainText(content) => BoxBody::new(content.clone()),
    })
}

#[async_trait]
#[provides]
impl GraphQLServer for GraphQLServerImpl {
    fn get_schema(&self) -> InexorSchema {
        Schema::build(InexorQuery, InexorMutation, InexorSubscription)
            .data(self.component_manager.clone())
            .data(self.entity_type_manager.clone())
            .data(self.relation_type_manager.clone())
            .data(self.entity_instance_manager.clone())
            .data(self.relation_instance_manager.clone())
            .data(self.flow_manager.clone())
            .finish()
    }

    // TODO: Extract to separate service: GraphQLQueryService <- Deno Integration
    async fn query(&self, request: String) -> Result<String, serde_json::Error> {
        info!("query");
        let schema = self.get_schema();
        let result = schema.execute(request).await;
        let json = serde_json::to_string(&result);
        match json {
            Ok(result) => Ok(result),
            Err(err) => Err(err),
        }
    }

    fn query_thread(&self, request: String) {
        let schema = self.get_schema();
        let _thread = task::Builder::new().name(String::from("query")).spawn(async move {
            info!("query: {}", request.clone());
            let result = schema.execute(request).await;
            let json = serde_json::to_string(&result);
            info!("query result: {}", json.unwrap());
        });
    }

    fn serve(&self, stopper: Receiver<()>) {
        // TEST THE GRAPHQL SCHEMA  --- TODO: remove
        let request = "{ types { entities(name:\"add\") { name } } }";
        self.query_thread(request.to_string());

        // GraphQL Schema
        let schema = self.get_schema();

        // REST SERVICES
        let component_manager = web::Data::new(self.component_manager.clone());
        let entity_type_manager = web::Data::new(self.entity_type_manager.clone());
        let relation_type_manager = web::Data::new(self.relation_type_manager.clone());
        let entity_instance_manager = web::Data::new(self.entity_instance_manager.clone());
        let relation_instance_manager = web::Data::new(self.relation_instance_manager.clone());
        let flow_manager = web::Data::new(self.flow_manager.clone());
        let web_resource_manager = web::Data::new(self.web_resource_manager.clone());
        let schema_data = web::Data::new(schema.clone());

        let system = actix::System::new(); // actix::System::new("inexor-graphql");

        let graphql_server_config = get_graphql_server_config();

        let mut http_server = HttpServer::new(move || {
            App::new()
                .wrap(Cors::permissive())
                .app_data(schema_data.clone())
                .app_data(component_manager.clone())
                .app_data(entity_type_manager.clone())
                .app_data(relation_type_manager.clone())
                .app_data(entity_instance_manager.clone())
                .app_data(relation_instance_manager.clone())
                .app_data(flow_manager.clone())
                .app_data(web_resource_manager.clone())
                // GraphQL API
                .service(query_graphql)
                .service(
                    web::resource("/graphql")
                        .guard(guard::Get())
                        .guard(guard::Header("upgrade", "websocket"))
                        .to(subscription_websocket),
                )
                // REST API
                .service(crate::rest::types::components::get_components)
                .service(crate::rest::types::entities::get_entity_types)
                .service(crate::rest::types::entities::get_entity_type)
                .service(crate::rest::types::relations::get_relation_types)
                .service(crate::rest::types::relations::get_relation_type)
                // Web Resource API
                .service(web::resource("/{web_resource_base_path}/{path:.*}").route(web::get().to(handle_web_resource)))
        })
        .disable_signals();

        if graphql_server_config.shutdown_timeout.is_some() {
            http_server = http_server.shutdown_timeout(graphql_server_config.shutdown_timeout.unwrap());
        }

        if graphql_server_config.workers.is_some() {
            http_server = http_server.workers(graphql_server_config.workers.unwrap());
        }

        debug!("Starting HTTP/GraphQL server on {}", graphql_server_config.to_string());
        let r_http_server = http_server.bind(graphql_server_config.to_string());
        if r_http_server.is_err() {
            error!("Could not start HTTP/GraphQL server: Failed to bind {}", graphql_server_config.to_string());
            return;
        }
        let http_server = r_http_server.unwrap();
        let server = http_server.run();
        let server_handle = server.handle();
        let t_server_handle = server_handle.clone();

        let terminate = Arc::new(AtomicBool::new(false));
        let t_terminate = terminate.clone();

        // This thread handles the server stop routine from the main thread
        std::thread::spawn(move || {
            // wait for shutdown signal
            stopper.recv().unwrap();
            debug!("Received shutdown signal. Stopping GraphQL server thread.");

            // stop server gracefully
            futures::executor::block_on(t_server_handle.stop(true));

            debug!("Successfully stopped GraphQL server thread.");
            t_terminate.store(true, Ordering::Relaxed);
            debug!("Stopping actix system.");
        });

        // This thread runs the GraphQL server
        match task::Builder::new().name(String::from("inexor-graphql")).spawn(server) {
            Ok(_join_handle) => {
                // Start the event loop
                system.block_on(async {
                    while !terminate.load(Ordering::Relaxed) {
                        thread::sleep(Duration::from_millis(100));
                    }
                    debug!("Successfully stopped the actix system.");
                });
            }
            Err(e) => {
                warn!("Failed to run actix system: {}", e);
            }
        }
    }
}

impl Lifecycle for GraphQLServerImpl {
    fn init(&self) {}

    fn post_init(&self) {}

    fn pre_shutdown(&self) {}

    fn shutdown(&self) {}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphSqlServerConfig {
    pub hostname: String,
    pub port: i32,
    pub shutdown_timeout: Option<u64>,
    pub workers: Option<usize>,
}

impl Default for GraphSqlServerConfig {
    fn default() -> Self {
        GraphSqlServerConfig {
            hostname: String::from("localhost"),
            port: 31415,
            shutdown_timeout: None,
            workers: None,
        }
    }
}

impl ToString for GraphSqlServerConfig {
    fn to_string(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}

fn get_graphql_server_config() -> GraphSqlServerConfig {
    // TODO: resolve config file from CONFIG_LOCATION(s)
    let toml_config = std::fs::read_to_string("./config/graphql.toml");
    match toml_config {
        Ok(toml_string) => {
            let graphql_server_config: Result<GraphSqlServerConfig, _> = toml::from_str(&toml_string);
            if graphql_server_config.is_err() {
                error!("Failed to load graphql configuration from {}: Invalid TOML", "./config/graphql.toml");
            }
            graphql_server_config.unwrap_or_default()
        }
        Err(_) => {
            error!("Failed to load graphql configuration from {}: File does not exist", "./config/graphql.toml");
            GraphSqlServerConfig::default()
        }
    }
}
