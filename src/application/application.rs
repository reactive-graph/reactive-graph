use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use crate::di::module;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use async_trait::async_trait;
use log::debug;
use log::info;

use crate::api::*;

#[wrapper]
pub struct RunningState(RwLock<bool>);

#[provides]
fn create_running_state_wrapper() -> RunningState {
    RunningState(RwLock::new(false))
}

#[async_trait]
pub trait Application: Send + Sync {
    //  + Lifecycle
    fn init(&self);

    fn post_init(&self);

    fn pre_shutdown(&self);

    fn shutdown(&self);

    async fn run(&mut self);

    fn stop(&self);

    fn is_running(&self) -> bool;

    fn get_component_behaviour_manager(&self) -> Arc<dyn ComponentBehaviourManager>;

    fn get_component_manager(&self) -> Arc<dyn ComponentManager>;

    // fn get_dynamic_graph(&self) -> Arc<dyn DynamicGraph>;

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager>;

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager>;

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager>;

    fn get_entity_vertex_manager(&self) -> Arc<dyn EntityVertexManager>;

    fn get_event_manager(&self) -> Arc<dyn SystemEventManager>;

    fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager>;

    fn get_graph_database(&self) -> Arc<dyn GraphDatabase>;

    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService>;

    fn get_graphql_schema_manager(&self) -> Arc<dyn GraphQLSchemaManager>;

    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer>;

    fn get_plugin_registry(&self) -> Arc<dyn PluginRegistry>;

    fn get_reactive_entity_instance_manager(&self) -> Arc<dyn ReactiveEntityInstanceManager>;

    fn get_reactive_flow_instance_manager(&self) -> Arc<dyn ReactiveFlowInstanceManager>;

    fn get_reactive_relation_instance_manager(&self) -> Arc<dyn ReactiveRelationInstanceManager>;

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager>;

    fn get_relation_edge_manager(&self) -> Arc<dyn RelationEdgeManager>;

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager>;

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager>;

    fn get_shutdown_manager(&self) -> Arc<dyn ShutdownManager>;

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager>;
}

#[module]
pub struct ApplicationImpl {
    running: RunningState,

    component_behaviour_manager: Wrc<dyn ComponentBehaviourManager>,
    component_manager: Wrc<dyn ComponentManager>,
    // dynamic_graph: Wrc<dyn DynamicGraph>,
    event_manager: Wrc<dyn SystemEventManager>,
    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,
    entity_instance_manager: Wrc<dyn EntityInstanceManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    entity_vertex_manager: Wrc<dyn EntityVertexManager>,
    flow_instance_manager: Wrc<dyn FlowInstanceManager>,
    graph_database: Wrc<dyn GraphDatabase>,
    graphql_query_service: Wrc<dyn GraphQLQueryService>,
    graphql_schema_manager: Wrc<dyn GraphQLSchemaManager>,
    graphql_server: Wrc<dyn GraphQLServer>,
    shutdown_manager: Wrc<dyn ShutdownManager>,
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,
    reactive_relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,
    reactive_flow_instance_manager: Wrc<dyn ReactiveFlowInstanceManager>,
    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,
    relation_edge_manager: Wrc<dyn RelationEdgeManager>,
    relation_instance_manager: Wrc<dyn RelationInstanceManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    plugin_registry: Wrc<dyn PluginRegistry>,
    web_resource_manager: Wrc<dyn WebResourceManager>,
}

#[async_trait]
#[provides]
impl Application for ApplicationImpl {
    fn init(&self) {
        // Order matters
        self.component_manager.init();
        self.entity_type_manager.init();
        self.relation_type_manager.init();
        self.plugin_registry.init();
        self.reactive_flow_instance_manager.init();
        self.web_resource_manager.init();
        self.graphql_schema_manager.init();
        self.graphql_query_service.init();
        self.graphql_server.init();
        self.shutdown_manager.init();
        self.event_manager.init();
        self.reactive_entity_instance_manager.init();
        // self.dynamic_graph.init();
    }

    fn post_init(&self) {
        // Order matters
        self.component_manager.post_init();
        self.entity_type_manager.post_init();
        self.relation_type_manager.post_init();
        self.plugin_registry.post_init();
        self.reactive_flow_instance_manager.post_init();
        self.web_resource_manager.post_init();
        self.graphql_schema_manager.post_init();
        self.graphql_query_service.post_init();
        self.graphql_server.post_init();
        self.shutdown_manager.post_init();
        self.event_manager.post_init();
        self.reactive_entity_instance_manager.post_init(); // after event_manager!
                                                           // self.dynamic_graph.post_init();
    }

    fn pre_shutdown(&self) {
        // Reverse order matters
        // self.dynamic_graph.pre_shutdown();
        self.reactive_entity_instance_manager.pre_shutdown();
        self.event_manager.pre_shutdown();
        self.shutdown_manager.pre_shutdown();
        self.graphql_server.pre_shutdown();
        self.graphql_query_service.pre_shutdown();
        self.graphql_schema_manager.pre_shutdown();
        self.web_resource_manager.pre_shutdown();
        self.reactive_flow_instance_manager.pre_shutdown();
        self.plugin_registry.pre_shutdown();
        self.relation_type_manager.pre_shutdown();
        self.entity_type_manager.pre_shutdown();
        self.component_manager.pre_shutdown();
    }

    fn shutdown(&self) {
        // Reverse order matters
        // self.dynamic_graph.shutdown();
        self.reactive_entity_instance_manager.shutdown();
        self.event_manager.shutdown();
        self.shutdown_manager.shutdown();
        self.graphql_server.shutdown();
        self.graphql_query_service.shutdown();
        self.graphql_schema_manager.shutdown();
        self.web_resource_manager.shutdown();
        self.reactive_flow_instance_manager.shutdown();
        self.plugin_registry.shutdown();
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
        let _graphql_server_stop_result = graphql_server_stop_sender.send(());

        // Be sure that the GraphQL server thread is gone
        if let Ok(thread_handle) = thread_handle {
            let _joined = thread_handle.join();
        }
        info!("Bye.");
    }

    fn stop(&self) {
        {
            let mut running = self.running.0.write().unwrap();
            *running = false;
        }
    }

    fn is_running(&self) -> bool {
        *self.running.0.read().unwrap().deref()
    }

    fn get_component_behaviour_manager(&self) -> Arc<dyn ComponentBehaviourManager> {
        self.component_behaviour_manager.clone()
    }

    fn get_component_manager(&self) -> Arc<dyn ComponentManager> {
        self.component_manager.clone()
    }

    // fn get_dynamic_graph(&self) -> Arc<dyn DynamicGraph> {
    //     self.dynamic_graph.clone()
    // }

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager> {
        self.entity_behaviour_manager.clone()
    }

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager> {
        self.entity_instance_manager.clone()
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager> {
        self.entity_type_manager.clone()
    }

    fn get_entity_vertex_manager(&self) -> Arc<dyn EntityVertexManager> {
        self.entity_vertex_manager.clone()
    }

    fn get_event_manager(&self) -> Arc<dyn SystemEventManager> {
        self.event_manager.clone()
    }

    fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager> {
        self.flow_instance_manager.clone()
    }

    fn get_graph_database(&self) -> Arc<dyn GraphDatabase> {
        self.graph_database.clone()
    }

    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService> {
        self.graphql_query_service.clone()
    }

    fn get_graphql_schema_manager(&self) -> Arc<dyn GraphQLSchemaManager> {
        self.graphql_schema_manager.clone()
    }

    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer> {
        self.graphql_server.clone()
    }

    fn get_plugin_registry(&self) -> Arc<dyn PluginRegistry> {
        self.plugin_registry.clone()
    }

    fn get_reactive_entity_instance_manager(&self) -> Arc<dyn ReactiveEntityInstanceManager> {
        self.reactive_entity_instance_manager.clone()
    }

    fn get_reactive_flow_instance_manager(&self) -> Arc<dyn ReactiveFlowInstanceManager> {
        self.reactive_flow_instance_manager.clone()
    }

    fn get_reactive_relation_instance_manager(&self) -> Arc<dyn ReactiveRelationInstanceManager> {
        self.reactive_relation_instance_manager.clone()
    }

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager> {
        self.relation_behaviour_manager.clone()
    }

    fn get_relation_edge_manager(&self) -> Arc<dyn RelationEdgeManager> {
        self.relation_edge_manager.clone()
    }

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager> {
        self.relation_instance_manager.clone()
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager> {
        self.relation_type_manager.clone()
    }

    fn get_shutdown_manager(&self) -> Arc<dyn ShutdownManager> {
        self.shutdown_manager.clone()
    }

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager> {
        self.web_resource_manager.clone()
    }
}
