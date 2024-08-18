use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::guard;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::ResponseError;
use actix_web::Result;
use actix_web_extras::middleware::Condition;
use async_trait::async_trait;
use crossbeam::channel::Receiver;
use log::debug;
use log::error;
use log::info;
use log::trace;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::ServerConfig;
use rustls_pemfile::certs;
use rustls_pemfile::read_all;
use rustls_pemfile::Item;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_config_api::ConfigManager;
use reactive_graph_dynamic_graph_api::DynamicGraphSchemaManager;
use reactive_graph_dynamic_graph_web::query_dynamic_graph;
use reactive_graph_graphql_api::GraphQLSchemaManager;
use reactive_graph_graphql_web::query_graphql;
use reactive_graph_graphql_web::subscription_websocket;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_graphql_api::PluginSchemaManager;
use reactive_graph_plugin_graphql_web::query_plugin_graphql;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_runtime_graphql_api::RuntimeSchemaManager;
use reactive_graph_runtime_graphql_web::query_runtime_graphql;
use reactive_graph_runtime_web_api::GraphQLServer;
use reactive_graph_runtime_web_api::WebResourceManager;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::get_logger_middleware;
use crate::web_resource_manager_handler::handle_root_web_resource;
use crate::web_resource_manager_handler::handle_web_resource;

#[derive(Debug, thiserror::Error)]
pub enum GraphQLServerError {
    #[error("The certificate chain is empty")]
    EmptyCertificateChain,
    #[error("No private key was found")]
    NoPrivateKeyFound,
    #[error("rustls error: {0}")]
    RustlsError(#[from] rustls::Error),
}

impl ResponseError for GraphQLServerError {}

#[derive(Component)]
pub struct GraphQLServerImpl {
    web_resource_manager: Arc<dyn WebResourceManager + Send + Sync>,

    config_manager: Arc<dyn ConfigManager + Send + Sync>,

    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,

    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,

    reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,

    reactive_relation_manager: Arc<dyn ReactiveRelationManager + Send + Sync>,

    reactive_flow_manager: Arc<dyn ReactiveFlowManager + Send + Sync>,

    graphql_schema_manager: Arc<dyn GraphQLSchemaManager + Send + Sync>,

    dynamic_graph_schema_manager: Arc<dyn DynamicGraphSchemaManager + Send + Sync>,

    runtime_schema_manager: Arc<dyn RuntimeSchemaManager + Send + Sync>,

    plugin_schema_manager: Arc<dyn PluginSchemaManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
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
        let schema_data = web::Data::new(schema);

        // Runtime GraphQL Schema
        let runtime_schema = self.runtime_schema_manager.get_schema();
        let runtime_schema_data = web::Data::new(runtime_schema);

        // Plugin GraphQL Schema
        let plugin_schema = self.plugin_schema_manager.get_schema();
        let plugin_schema_data = web::Data::new(plugin_schema);

        // REST SERVICES
        let component_manager = web::Data::new(self.component_manager.clone());
        let entity_type_manager = web::Data::new(self.entity_type_manager.clone());
        let relation_type_manager = web::Data::new(self.relation_type_manager.clone());
        let flow_type_manager = web::Data::new(self.flow_type_manager.clone());

        let entity_instance_manager = web::Data::new(self.reactive_entity_manager.clone());
        let relation_instance_manager = web::Data::new(self.reactive_relation_manager.clone());
        let flow_instance_manager = web::Data::new(self.reactive_flow_manager.clone());

        let web_resource_manager = web::Data::new(self.web_resource_manager.clone());

        let dynamic_graph_schema_manager = web::Data::new(self.dynamic_graph_schema_manager.clone());

        let http_server = HttpServer::new(move || {
            let graphql_logging_config = graphql_logging_config.clone();
            App::new()
                .wrap(Cors::permissive())
                .wrap(Condition::from_option(get_logger_middleware(&graphql_logging_config)))
                // Type System
                .app_data(component_manager.clone())
                .app_data(entity_type_manager.clone())
                .app_data(relation_type_manager.clone())
                .app_data(flow_type_manager.clone())
                // Instance System
                .app_data(entity_instance_manager.clone())
                .app_data(relation_instance_manager.clone())
                .app_data(flow_instance_manager.clone())
                // Web Resources
                .app_data(web_resource_manager.clone())
                // GraphQL Schema
                .app_data(schema_data.clone())
                // Runtime GraphQL Schema
                .app_data(runtime_schema_data.clone())
                // Plugin GraphQL Schema
                .app_data(plugin_schema_data.clone())
                // Dynamic Graph
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
                // Runtime GraphQL API
                .service(query_runtime_graphql)
                .service(query_plugin_graphql)
                // Type System REST API
                .service(reactive_graph_type_system_rest::components::get_components)
                .service(reactive_graph_type_system_rest::components::get_component)
                .service(reactive_graph_type_system_rest::entities::get_entity_types)
                .service(reactive_graph_type_system_rest::entities::get_entity_type)
                .service(reactive_graph_type_system_rest::relations::get_relation_types)
                .service(reactive_graph_type_system_rest::relations::get_relation_type)
                .service(reactive_graph_type_system_rest::flows::get_flow_types)
                .service(reactive_graph_type_system_rest::flows::get_flow_type)
                // JSON Schema
                .service(reactive_graph_type_system_json_schema::types::components::schema_components)
                .service(reactive_graph_type_system_json_schema::types::entities::schema_entity_types)
                .service(reactive_graph_type_system_json_schema::types::relations::schema_relation_types)
                .service(reactive_graph_type_system_json_schema::types::flows::schema_flow_types)
                .service(reactive_graph_type_system_json_schema::instances::entities::schema_entity_instances)
                .service(reactive_graph_type_system_json_schema::instances::relations::schema_relation_instances)
                .service(reactive_graph_type_system_json_schema::instances::flows::schema_flow_instances)
                // Web Resource API
                .service(web::resource("/{web_resource_context_path}/{path:.*}").route(web::get().to(handle_web_resource)))
                .service(web::resource("/{path:.*}").route(web::get().to(handle_root_web_resource)))
        })
        .disable_signals()
        .shutdown_timeout(graphql_server_config.shutdown_timeout())
        .workers(graphql_server_config.workers());

        let r_http_server = if graphql_server_config.is_secure() {
            let cert_file = File::open(graphql_server_config.ssl_certificate_path())?;
            let cert_file = &mut BufReader::new(cert_file);
            let cert_chain: Vec<CertificateDer> = certs(cert_file).filter_map(|cert| cert.ok()).collect();
            if cert_chain.is_empty() {
                return Err(GraphQLServerError::EmptyCertificateChain.into());
            }

            let key_file = File::open(graphql_server_config.ssl_private_key_path())?;
            let key_file = &mut BufReader::new(key_file);
            let mut keys: Vec<PrivateKeyDer> = read_all(key_file)
                .filter_map(|item| match item {
                    Ok(Item::Pkcs1Key(key)) => Some(key.into()),
                    Ok(Item::Pkcs8Key(key)) => Some(key.into()),
                    Ok(Item::Sec1Key(key)) => Some(key.into()),
                    Ok(_) => {
                        error!("Could not load private key: The file does not contain a private key in either format PKCS1, PKCS8, SEC1!");
                        None
                    }
                    Err(e) => {
                        error!("Failed to load private key: {e}");
                        None
                    }
                })
                .collect();
            if keys.is_empty() {
                error!("Could not load private keys.");
                return Err(GraphQLServerError::NoPrivateKeyFound.into());
            }
            let tls_config = ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(cert_chain, keys.remove(0))
                .map_err(|e| GraphQLServerError::RustlsError(e))?;
            info!("Starting HTTPS/GraphQL server on {}", graphql_server_config.url());
            http_server.bind_rustls_0_23(graphql_server_config.addr(), tls_config)?.run()
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
