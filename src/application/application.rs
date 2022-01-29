use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::Duration;

use async_trait::async_trait;
use log::{debug, info};
use waiter_di::*;

use crate::api::*;

#[wrapper]
pub struct RunningState(RwLock<bool>);

#[provides]
fn create_external_type_dependency() -> RunningState {
    RunningState(RwLock::new(false))
}

#[async_trait]
pub trait Application: Send + Sync {
    //  + Lifecycle
    fn init(&self);

    fn shutdown(&self);

    async fn run(&mut self);

    fn stop(&self);

    fn is_running(&self) -> bool;

    fn get_shutdown_manager(&self) -> Arc<dyn ShutdownManager>;

    fn get_graph_database(&self) -> Arc<dyn GraphDatabase>;

    fn get_component_manager(&self) -> Arc<dyn ComponentManager>;

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager>;

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager>;

    fn get_entity_vertex_manager(&self) -> Arc<dyn EntityVertexManager>;

    fn get_relation_edge_manager(&self) -> Arc<dyn RelationEdgeManager>;

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager>;

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager>;

    fn get_flow_manager(&self) -> Arc<dyn FlowManager>;

    fn get_reactive_entity_instance_manager(&self) -> Arc<dyn ReactiveEntityInstanceManager>;

    fn get_reactive_relation_instance_manager(&self) -> Arc<dyn ReactiveRelationInstanceManager>;

    fn get_reactive_flow_manager(&self) -> Arc<dyn ReactiveFlowManager>;

    fn get_component_behaviour_manager(&self) -> Arc<dyn ComponentBehaviourManager>;

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager>;

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager>;

    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer>;

    fn get_plugin_registry(&self) -> Arc<dyn PluginRegistry>;

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager>;
}

#[module]
pub struct ApplicationImpl {
    running: RunningState,

    shutdown_manager: Wrc<dyn ShutdownManager>,
    graph_database: Wrc<dyn GraphDatabase>,
    component_behaviour_manager: Wrc<dyn ComponentBehaviourManager>,
    component_manager: Wrc<dyn ComponentManager>,
    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,
    entity_instance_manager: Wrc<dyn EntityInstanceManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    entity_vertex_manager: Wrc<dyn EntityVertexManager>,
    flow_manager: Wrc<dyn FlowManager>,
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,
    reactive_relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,
    reactive_flow_manager: Wrc<dyn ReactiveFlowManager>,
    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,
    relation_edge_manager: Wrc<dyn RelationEdgeManager>,
    relation_instance_manager: Wrc<dyn RelationInstanceManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    graphql_server: Wrc<dyn GraphQLServer>,
    plugin_registry: Wrc<dyn PluginRegistry>,
    web_resource_manager: Wrc<dyn WebResourceManager>,
}

#[async_trait]
#[provides]
impl Application for ApplicationImpl {
    fn init(&self) {
        self.component_manager.init();
        self.entity_type_manager.init();
        self.relation_type_manager.init();
        self.plugin_registry.init();
        self.reactive_flow_manager.init();
        self.web_resource_manager.init();
        self.graphql_server.init();
        self.shutdown_manager.init();
    }

    fn shutdown(&self) {
        self.shutdown_manager.shutdown();
        self.graphql_server.shutdown();
        self.web_resource_manager.init();
        self.reactive_flow_manager.shutdown();
        self.plugin_registry.unload_plugins();
        self.relation_type_manager.shutdown();
        self.entity_type_manager.shutdown();
        self.component_manager.shutdown();
    }

    async fn run(&mut self) {
        // Signal handling
        let terminate = Arc::new(AtomicBool::new(false));
        // This channel allows the main thread to stop the GraphQL server thread
        let (graphql_server_stop_sender, graphql_server_stop_receiver) = mpsc::channel::<()>();
        // This channel allows the GraphQL server thread to tell the main thread that it has been finished
        let (graphql_server_stopped_sender, graphql_server_stopped_receiver) = mpsc::channel::<()>();
        // Clone GraphQL server and move the reference into the GraphQL server thread
        let graphql_server = self.graphql_server.clone();
        // GraphQL server thread: Create a new thread for the GraphQL server
        let thread_handle = thread::Builder::new()
            // Thread name
            .name(String::from("inexor-graphql"))
            // Move GraphQL server service reference into thread
            .spawn(move || {
                // Run the GraphQL server
                graphql_server.serve(graphql_server_stop_receiver);
                debug!("Successfully stopped GraphQL Server.");
                // Tell the main thread, that the GraphQL server thread has finished
                let _result = graphql_server_stopped_sender.send(());
            });

        {
            let mut running = self.running.0.write().unwrap();
            *running = true;
        }
        {
            let running = self.running.0.read().unwrap();

            let _r_sigint = signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate));
            let _r_sigterm = signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&terminate));

            let mut stopping = false;
            while *running && !stopping && !terminate.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100)); // from_millis(1)
                let r = graphql_server_stopped_receiver.try_recv();
                if r.is_ok() {
                    debug!("Stopping the main thread");
                    stopping = true;
                }
                if self.shutdown_manager.is_shutdown() {
                    stopping = true;
                }
            }
        } // Drop "running"

        // Stop GraphQL server thread, if it is still running
        debug!("Stopping the GraphQL server thread");
        let graphql_server_stop_result = graphql_server_stop_sender.send(());
        if graphql_server_stop_result.is_ok() {
            graphql_server_stop_result.unwrap();
        }

        // Be sure that the GraphQL server thread is gone
        if thread_handle.is_ok() {
            let _joined = thread_handle.unwrap().join();
        }
        info!("Bye.");
    }

    // TODO: remove the shared thread mechanics
    fn stop(&self) {
        {
            let mut running = self.running.0.write().unwrap();
            *running = false;
        }
    }

    fn is_running(&self) -> bool {
        *self.running.0.read().unwrap().deref()
    }

    fn get_shutdown_manager(&self) -> Arc<dyn ShutdownManager> {
        self.shutdown_manager.clone()
    }

    fn get_graph_database(&self) -> Arc<dyn GraphDatabase> {
        self.graph_database.clone()
    }

    fn get_component_manager(&self) -> Arc<dyn ComponentManager> {
        self.component_manager.clone()
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager> {
        self.entity_type_manager.clone()
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager> {
        self.relation_type_manager.clone()
    }

    fn get_entity_vertex_manager(&self) -> Arc<dyn EntityVertexManager> {
        self.entity_vertex_manager.clone()
    }

    fn get_relation_edge_manager(&self) -> Arc<dyn RelationEdgeManager> {
        self.relation_edge_manager.clone()
    }

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager> {
        self.entity_instance_manager.clone()
    }

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager> {
        self.relation_instance_manager.clone()
    }

    fn get_flow_manager(&self) -> Arc<dyn FlowManager> {
        self.flow_manager.clone()
    }

    fn get_reactive_entity_instance_manager(&self) -> Arc<dyn ReactiveEntityInstanceManager> {
        self.reactive_entity_instance_manager.clone()
    }

    fn get_reactive_relation_instance_manager(&self) -> Arc<dyn ReactiveRelationInstanceManager> {
        self.reactive_relation_instance_manager.clone()
    }

    fn get_reactive_flow_manager(&self) -> Arc<dyn ReactiveFlowManager> {
        self.reactive_flow_manager.clone()
    }

    fn get_component_behaviour_manager(&self) -> Arc<dyn ComponentBehaviourManager> {
        self.component_behaviour_manager.clone()
    }

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager> {
        self.entity_behaviour_manager.clone()
    }

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager> {
        self.relation_behaviour_manager.clone()
    }

    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer> {
        self.graphql_server.clone()
    }

    fn get_plugin_registry(&self) -> Arc<dyn PluginRegistry> {
        self.plugin_registry.clone()
    }

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager> {
        self.web_resource_manager.clone()
    }
}
