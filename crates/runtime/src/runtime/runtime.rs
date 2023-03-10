use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use crate::di::module;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use async_trait::async_trait;
use log::debug;
use log::info;
use tokio::time::error::Elapsed;

use crate::api::*;

#[wrapper]
pub struct RunningState(RwLock<bool>);

#[provides]
fn create_running_state_wrapper() -> RunningState {
    RunningState(RwLock::new(false))
}

#[async_trait]
pub trait Runtime: Send + Sync {
    //  + Lifecycle
    fn init(&self);

    fn post_init(&self);

    fn pre_shutdown(&self);

    fn shutdown(&self);

    async fn run(&self);

    fn stop(&self);

    fn is_running(&self) -> bool;

    async fn wait_for(&self, timeout_duration: Duration) -> Result<(), Elapsed>;

    fn get_component_manager(&self) -> Arc<dyn ComponentManager>;

    fn get_dynamic_graph_query_service(&self) -> Arc<dyn DynamicGraphQueryService>;

    fn get_dynamic_graph_schema_manager(&self) -> Arc<dyn DynamicGraphSchemaManager>;

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager>;

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry>;

    fn get_entity_component_behaviour_manager(&self) -> Arc<dyn EntityComponentBehaviourManager>;

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry>;

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager>;

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager>;

    fn get_entity_vertex_manager(&self) -> Arc<dyn EntityVertexManager>;

    fn get_event_manager(&self) -> Arc<dyn SystemEventManager>;

    fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager>;

    fn get_graph_database(&self) -> Arc<dyn GraphDatabase>;

    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService>;

    fn get_graphql_schema_manager(&self) -> Arc<dyn GraphQLSchemaManager>;

    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer>;

    fn get_instance_service(&self) -> Arc<dyn InstanceService>;

    fn get_namespace_manager(&self) -> Arc<dyn NamespaceManager>;

    fn get_plugin_container_manager(&self) -> Arc<dyn PluginContainerManager>;

    fn get_plugin_repository_manager(&self) -> Arc<dyn PluginRepositoryManager>;

    fn get_reactive_entity_instance_manager(&self) -> Arc<dyn ReactiveEntityInstanceManager>;

    fn get_reactive_flow_instance_manager(&self) -> Arc<dyn ReactiveFlowInstanceManager>;

    fn get_reactive_relation_instance_manager(&self) -> Arc<dyn ReactiveRelationInstanceManager>;

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager>;

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry>;

    fn get_relation_component_behaviour_manager(&self) -> Arc<dyn RelationComponentBehaviourManager>;

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry>;

    fn get_relation_edge_manager(&self) -> Arc<dyn RelationEdgeManager>;

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager>;

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager>;

    fn get_shutdown_manager(&self) -> Arc<dyn ShutdownManager>;

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager>;
}

#[module]
pub struct RuntimeImpl {
    running: RunningState,

    component_manager: Wrc<dyn ComponentManager>,
    dynamic_graph_query_service: Wrc<dyn DynamicGraphQueryService>,
    dynamic_graph_schema_manager: Wrc<dyn DynamicGraphSchemaManager>,
    event_manager: Wrc<dyn SystemEventManager>,
    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,
    entity_behaviour_registry: Wrc<dyn EntityBehaviourRegistry>,
    entity_component_behaviour_manager: Wrc<dyn EntityComponentBehaviourManager>,
    entity_component_behaviour_registry: Wrc<dyn EntityComponentBehaviourRegistry>,
    entity_instance_manager: Wrc<dyn EntityInstanceManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    entity_vertex_manager: Wrc<dyn EntityVertexManager>,
    flow_instance_manager: Wrc<dyn FlowInstanceManager>,
    graph_database: Wrc<dyn GraphDatabase>,
    graphql_query_service: Wrc<dyn GraphQLQueryService>,
    graphql_schema_manager: Wrc<dyn GraphQLSchemaManager>,
    graphql_server: Wrc<dyn GraphQLServer>,
    instance_service: Wrc<dyn InstanceService>,
    namespace_manager: Wrc<dyn NamespaceManager>,
    shutdown_manager: Wrc<dyn ShutdownManager>,
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,
    reactive_relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,
    reactive_flow_instance_manager: Wrc<dyn ReactiveFlowInstanceManager>,
    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,
    relation_behaviour_registry: Wrc<dyn RelationBehaviourRegistry>,
    relation_component_behaviour_manager: Wrc<dyn RelationComponentBehaviourManager>,
    relation_component_behaviour_registry: Wrc<dyn RelationComponentBehaviourRegistry>,
    relation_edge_manager: Wrc<dyn RelationEdgeManager>,
    relation_instance_manager: Wrc<dyn RelationInstanceManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    plugin_container_manager: Wrc<dyn PluginContainerManager>,
    plugin_context_factory: Wrc<dyn PluginContextFactory>,
    plugin_repository_manager: Wrc<dyn PluginRepositoryManager>,
    plugin_resolver: Wrc<dyn PluginResolver>,
    web_resource_manager: Wrc<dyn WebResourceManager>,
}

#[async_trait]
#[provides]
impl Runtime for RuntimeImpl {
    fn init(&self) {
        // Order matters
        self.component_manager.init();
        self.entity_type_manager.init();
        self.relation_type_manager.init();
        self.plugin_context_factory.init();
        self.plugin_repository_manager.init();
        self.plugin_resolver.init();
        self.reactive_flow_instance_manager.init();
        self.web_resource_manager.init();
        self.graphql_schema_manager.init();
        self.graphql_query_service.init();
        self.graphql_server.init();
        self.shutdown_manager.init();
        self.event_manager.init();
        self.reactive_entity_instance_manager.init();
        self.dynamic_graph_schema_manager.init();
        self.dynamic_graph_query_service.init();
    }

    fn post_init(&self) {
        // Order matters
        self.component_manager.post_init();
        self.entity_type_manager.post_init();
        self.relation_type_manager.post_init();
        self.plugin_context_factory.post_init();
        self.plugin_repository_manager.post_init();
        self.plugin_resolver.post_init();
        self.reactive_flow_instance_manager.post_init();
        self.web_resource_manager.post_init();
        self.graphql_schema_manager.post_init();
        self.graphql_query_service.post_init();
        self.graphql_server.post_init();
        self.shutdown_manager.post_init();
        self.event_manager.post_init();
        self.reactive_entity_instance_manager.post_init(); // after event_manager!
        self.dynamic_graph_schema_manager.post_init();
        self.dynamic_graph_query_service.post_init();
    }

    fn pre_shutdown(&self) {
        // Reverse order matters
        self.dynamic_graph_query_service.pre_shutdown();
        self.dynamic_graph_schema_manager.pre_shutdown();
        self.reactive_entity_instance_manager.pre_shutdown();
        self.event_manager.pre_shutdown();
        self.shutdown_manager.pre_shutdown();
        self.graphql_server.pre_shutdown();
        self.graphql_query_service.pre_shutdown();
        self.graphql_schema_manager.pre_shutdown();
        self.web_resource_manager.pre_shutdown();
        self.reactive_flow_instance_manager.pre_shutdown();
        self.plugin_resolver.pre_shutdown();
        self.plugin_repository_manager.pre_shutdown();
        self.plugin_context_factory.pre_shutdown();
        self.relation_type_manager.pre_shutdown();
        self.entity_type_manager.pre_shutdown();
        self.component_manager.pre_shutdown();
    }

    fn shutdown(&self) {
        // Reverse order matters
        self.dynamic_graph_query_service.shutdown();
        self.dynamic_graph_schema_manager.shutdown();
        self.reactive_entity_instance_manager.shutdown();
        self.event_manager.shutdown();
        self.shutdown_manager.shutdown();
        self.graphql_server.shutdown();
        self.graphql_query_service.shutdown();
        self.graphql_schema_manager.shutdown();
        self.web_resource_manager.shutdown();
        self.reactive_flow_instance_manager.shutdown();
        self.plugin_resolver.shutdown();
        self.plugin_repository_manager.shutdown();
        self.plugin_context_factory.shutdown();
        self.relation_type_manager.shutdown();
        self.entity_type_manager.shutdown();
        self.component_manager.shutdown();
    }

    async fn run(&self) {
        // Signal handling
        let terminate = Arc::new(AtomicBool::new(false));
        // This channel allows the main thread to stop the GraphQL server thread
        let (graphql_server_stop_sender, graphql_server_stop_receiver) = crossbeam::channel::unbounded::<()>();
        // This channel allows the GraphQL server thread to tell the main thread that it has been finished
        let (graphql_server_stopped_sender, graphql_server_stopped_receiver) = crossbeam::channel::unbounded::<()>();
        // Clone GraphQL server and move the reference into the GraphQL server thread
        let graphql_server = self.graphql_server.clone();
        // GraphQL server thread: Create a new thread for the GraphQL server
        // TODO: add thread name
        let graphql_server_handle = tokio::spawn(async move {
            // Run the GraphQL server
            info!("Run the GraphQL server.");
            graphql_server.serve(graphql_server_stop_receiver).await;
            debug!("Successfully stopped GraphQL Server.");
            // Tell the main thread, that the GraphQL server thread has finished
            let _result = graphql_server_stopped_sender.send(());
        });

        {
            let mut running = self.running.0.write().unwrap();
            *running = true;
        }

        {
            let _r_sigint = signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate));
            let _r_sigterm = signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&terminate));

            let mut stopping = false;
            while self.is_running() && !stopping && !terminate.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_millis(100)).await;
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
        let _ = graphql_server_handle.await;
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

    async fn wait_for(&self, timeout_duration: Duration) -> Result<(), Elapsed> {
        tokio::time::timeout(timeout_duration, self.wait_for_internal()).await
    }

    fn get_component_manager(&self) -> Arc<dyn ComponentManager> {
        self.component_manager.clone()
    }

    fn get_dynamic_graph_query_service(&self) -> Arc<dyn DynamicGraphQueryService> {
        self.dynamic_graph_query_service.clone()
    }

    fn get_dynamic_graph_schema_manager(&self) -> Arc<dyn DynamicGraphSchemaManager> {
        self.dynamic_graph_schema_manager.clone()
    }

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager> {
        self.entity_behaviour_manager.clone()
    }

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry> {
        self.entity_behaviour_registry.clone()
    }

    fn get_entity_component_behaviour_manager(&self) -> Arc<dyn EntityComponentBehaviourManager> {
        self.entity_component_behaviour_manager.clone()
    }

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry> {
        self.entity_component_behaviour_registry.clone()
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

    fn get_instance_service(&self) -> Arc<dyn InstanceService> {
        self.instance_service.clone()
    }

    fn get_namespace_manager(&self) -> Arc<dyn NamespaceManager> {
        self.namespace_manager.clone()
    }

    fn get_plugin_container_manager(&self) -> Arc<dyn PluginContainerManager> {
        self.plugin_container_manager.clone()
    }

    fn get_plugin_repository_manager(&self) -> Arc<dyn PluginRepositoryManager> {
        self.plugin_repository_manager.clone()
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

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry> {
        self.relation_behaviour_registry.clone()
    }

    fn get_relation_component_behaviour_manager(&self) -> Arc<dyn RelationComponentBehaviourManager> {
        self.relation_component_behaviour_manager.clone()
    }

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry> {
        self.relation_component_behaviour_registry.clone()
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

impl RuntimeImpl {
    async fn wait_for_internal(&self) {
        while !self.is_running() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
