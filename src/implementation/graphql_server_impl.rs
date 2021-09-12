use std::sync::mpsc::Receiver;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{post, web, App, HttpServer};
use async_graphql::*;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use async_std::task;
use async_trait::async_trait;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use waiter_di::*;

use crate::api::{
    ComponentManager, EntityTypeManager, GraphQLServer, Lifecycle, ReactiveEntityInstanceManager,
    ReactiveFlowManager, ReactiveRelationInstanceManager, RelationTypeManager,
};
use crate::graphql::{InexorMutation, InexorQuery, InexorSchema};

#[component]
pub struct GraphQLServerImpl {
    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,

    flow_manager: Wrc<dyn ReactiveFlowManager>,
}

#[post("/graphql")]
async fn query_graphql(schema: web::Data<InexorSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

#[async_trait]
#[provides]
impl GraphQLServer for GraphQLServerImpl {
    fn get_schema(&self) -> InexorSchema {
        Schema::build(InexorQuery, InexorMutation, EmptySubscription)
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
            Err(err) => Err(err.into()),
        }
    }

    fn query_thread(&self, request: String) {
        let schema = self.get_schema();
        let _thread = task::Builder::new()
            .name(String::from("query"))
            .spawn(async move {
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

        let system = actix::System::new(); // actix::System::new("inexor-graphql");

        let server = HttpServer::new(move || {
            App::new()
                .wrap(Cors::permissive())
                .data(schema.clone())
                .app_data(component_manager.clone())
                .app_data(entity_type_manager.clone())
                .app_data(relation_type_manager.clone())
                .app_data(entity_instance_manager.clone())
                .app_data(relation_instance_manager.clone())
                .app_data(flow_manager.clone())
                // GraphQL API
                .service(query_graphql)
                // REST API
                .service(crate::rest::types::components::get_components)
                .service(crate::rest::types::entities::get_entity_types)
                .service(crate::rest::types::entities::get_entity_type)
                .service(crate::rest::types::relations::get_relation_types)
                .service(crate::rest::types::relations::get_relation_type)
                // TODO: query instances
                // TODO: modify types
                // TODO: modify instances
                // TODO: query flows
                // TODO: modify flows
                // Flow Designer
                .service(
                    fs::Files::new("/flow-designer", "./web/flow-designer")
                        .index_file("index.html"),
                )
                // Main User Interface
                .service(fs::Files::new("/", "./web/user-interface").index_file("index.html"))
        });

        let graphql_server_config = get_graphql_server_config();
        debug!(
            "Starting HTTP/GraphQL server on {}",
            graphql_server_config.to_string()
        );
        let r_server = server.bind(graphql_server_config.to_string());
        if r_server.is_err() {
            error!(
                "Could not start HTTP/GraphQL server: Failed to bind {}",
                graphql_server_config.to_string()
            );
            return;
        }
        let server = r_server.unwrap().run();

        // This thread handles the server stop routine from the main thread
        let srv = server.clone();
        std::thread::spawn(move || {
            // wait for shutdown signal
            stopper.recv().unwrap();

            // stop server gracefully
            futures::executor::block_on(srv.stop(true))
        });

        // This thread runs the GraphQL server
        let handle = task::Builder::new()
            .name(String::from("inexor-graphql"))
            .spawn(server);
        if handle.is_ok() {
            let _handle = handle.unwrap();
            // Start the event loop
            let _ = system.run();
        }
    }
}

impl Lifecycle for GraphQLServerImpl {
    fn init(&self) {}

    fn shutdown(&self) {}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphSqlServerConfig {
    pub hostname: String,
    pub port: i32,
}

impl Default for GraphSqlServerConfig {
    fn default() -> Self {
        GraphSqlServerConfig {
            hostname: String::from("localhost"),
            port: 31415,
        }
    }
}

impl ToString for GraphSqlServerConfig {
    fn to_string(&self) -> String {
        String::from(format!("{}:{}", self.hostname, self.port))
    }
}

fn get_graphql_server_config() -> GraphSqlServerConfig {
    let toml_config = std::fs::read_to_string("./config/graphql.toml");
    match toml_config {
        Ok(toml_string) => {
            let graphql_server_config: Result<GraphSqlServerConfig, _> =
                toml::from_str(&toml_string);
            if graphql_server_config.is_err() {
                error!(
                    "Failed to load graphql configuration from {}: Invalid TOML",
                    "./config/graphql.toml"
                );
            }
            graphql_server_config.unwrap_or(GraphSqlServerConfig::default())
        }
        Err(_) => {
            error!(
                "Failed to load graphql configuration from {}: File does not exist",
                "./config/graphql.toml"
            );
            GraphSqlServerConfig::default()
        }
    }
}
