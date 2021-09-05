use std::ops::Deref;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::Duration;

use async_trait::async_trait;
use log::debug;
use waiter_di::*;

use crate::api::*;
use crate::plugins::Plugin;

// #[cfg(feature = "arithmetic")]
// use inexor_ecs_type_system_arithmetic::subsystem::get_arithmetic_subsystem;
//
// #[cfg(feature = "base")]
// use inexor_ecs_type_system_base::subsystem::get_base_subsystem;
//
// #[cfg(feature = "connector")]
// use inexor_ecs_type_system_connector::subsystem::get_connector_subsystem;
//
// #[cfg(feature = "inout")]
// use inexor_ecs_type_system_inout::subsystem::get_inout_subsystem;
//
// #[cfg(feature = "logical")]
// use inexor_ecs_type_system_logical::subsystem::get_logical_subsystem;
//
// #[cfg(feature = "mqtt")]
// use inexor_ecs_type_system_mqtt::subsystem::get_mqtt_subsystem;
//
// #[cfg(feature = "numeric")]
// use inexor_ecs_type_system_numeric::subsystem::get_numeric_subsystem;
//
// #[cfg(feature = "system_environment")]
// use inexor_system_environment::subsystem::get_system_environment_subsystem;
//
// #[cfg(feature = "taxonomy")]
// use inexor_ecs_type_system_taxonomy::subsystem::get_taxonomy_subsystem;
//
// #[cfg(feature = "user_interface")]
// use inexor_ecs_type_system_ui::subsystem::get_user_interface_subsystem;
//
// #[cfg(feature = "value")]
// use inexor_ecs_type_system_value::subsystem::get_value_subsystem;

#[wrapper]
pub struct RunningState(RwLock<bool>);

#[provides]
fn create_external_type_dependency() -> RunningState {
    RunningState(RwLock::new(false))
}

pub struct Plugins {
    plugins: Vec<Arc<dyn Plugin>>,
}

#[wrapper]
pub struct PluginsWrapper(Plugins);

#[provides]
fn create_plugins() -> PluginsWrapper {
    PluginsWrapper(Plugins {
        plugins: vec![
            // TODO: Migrate to plugins
            // TODO: Ordering
            #[cfg(feature = "base")]
            get_base_subsystem().unwrap(),
            #[cfg(feature = "connector")]
            get_connector_subsystem().unwrap(),
            #[cfg(feature = "value")]
            get_value_subsystem().unwrap(),
            #[cfg(feature = "arithmetic")]
            get_arithmetic_subsystem().unwrap(),
            #[cfg(feature = "numeric")]
            get_numeric_subsystem().unwrap(),
            #[cfg(feature = "logical")]
            get_logical_subsystem().unwrap(),
            #[cfg(feature = "inout")]
            get_inout_subsystem().unwrap(),
            #[cfg(feature = "mqtt")]
            get_mqtt_subsystem().unwrap(),
            #[cfg(feature = "system_environment")]
            get_system_environment_subsystem().unwrap(),
            #[cfg(feature = "taxonomy")]
            get_taxonomy_subsystem().unwrap(),
            #[cfg(feature = "user_interface")]
            get_user_interface_subsystem().unwrap(),
        ],
    })
}

#[async_trait]
pub trait Application: Send + Sync {
    //  + Lifecycle
    fn init(&self);

    fn shutdown(&self);

    async fn run(&mut self);

    fn stop(&self);

    fn is_running(&self) -> bool;

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

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager>;

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager>;

    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer>;

    // fn get_subsystem(&self) -> Arc<dyn Plugin>;
}

#[module]
pub struct ApplicationImpl {
    running: RunningState,

    graph_database: Wrc<dyn GraphDatabase>,
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

    // system_constants_initializer: Wrc<dyn SystemConstantsInitializer>,
    graphql_server: Wrc<dyn GraphQLServer>,

    plugins: PluginsWrapper,
}

#[async_trait]
#[provides]
impl Application for ApplicationImpl {
    fn init(&self) {
        self.component_manager.init();
        self.entity_type_manager.init();
        self.relation_type_manager.init();
        // TODO: Move the plugin initialization
        // TODO: Add methods to load and unload plugins
        for sub_system in self.plugins.0.plugins.iter() {
            sub_system.init();
            match sub_system.get_component_provider() {
                Ok(component_provider) => self.component_manager.add_provider(component_provider),
                Err(_) => {}
            }
            match sub_system.get_entity_type_provider() {
                Ok(entity_type_provider) => {
                    self.entity_type_manager.add_provider(entity_type_provider)
                }
                Err(_) => {}
            }
            match sub_system.get_relation_type_provider() {
                Ok(relation_type_provider) => self
                    .relation_type_manager
                    .add_provider(relation_type_provider),
                Err(_) => {}
            }
            match sub_system.get_entity_behaviour_provider() {
                Ok(entity_behaviour_provider) => self
                    .entity_behaviour_manager
                    .add_provider(entity_behaviour_provider),
                Err(_) => {}
            }
            match sub_system.get_relation_behaviour_provider() {
                Ok(relation_behaviour_provider) => self
                    .relation_behaviour_manager
                    .add_provider(relation_behaviour_provider),
                Err(_) => {}
            }
            match sub_system.get_flow_provider() {
                Ok(flow_provider) => self.reactive_flow_manager.add_provider(flow_provider),
                Err(_) => {}
            }
            sub_system.post_init();
        }
        self.reactive_flow_manager.init();
        self.graphql_server.init();
        // TODO: Migrate system_constants_initializer to submodule
        // self.system_constants_initializer.init();
    }

    fn shutdown(&self) {
        // TODO: Migrate system_constants_initializer to submodule
        // self.system_constants_initializer.shutdown();

        self.graphql_server.shutdown();
        self.reactive_flow_manager.shutdown();
        for sub_system in self.plugins.0.plugins.iter().rev() {
            sub_system.shutdown();
        }
        self.relation_type_manager.shutdown();
        self.entity_type_manager.shutdown();
        self.component_manager.shutdown();
    }

    async fn run(&mut self) {
        // This channel allows the main thread to stop the GraphQL server thread
        let (graphql_server_stop_sender, graphql_server_stop_receiver) = mpsc::channel::<()>();
        // This channel allows the GraphQL server thread to tell the main thread that it has been finished
        let (graphql_server_stopped_sender, graphql_server_stopped_receiver) =
            mpsc::channel::<()>();
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
                // Tell the main thread, that the GraphQL server thread has finished
                let result = graphql_server_stopped_sender.send(());
                if result.is_ok() {
                    result.unwrap();
                }
            });

        {
            let mut running = self.running.0.write().unwrap();
            *running = true;
        }
        {
            let running = self.running.0.read().unwrap();

            let mut stopping = false;
            while *running && !stopping {
                thread::sleep(Duration::from_millis(100)); // from_millis(1)
                let r = graphql_server_stopped_receiver.try_recv();
                if r.is_ok() {
                    debug!("Stopping the main thread");
                    stopping = true;
                }
            }
        } // Drop "running"

        // Stop GraphQL server thread, if it is still running
        let graphql_server_stop_result = graphql_server_stop_sender.send(());
        if graphql_server_stop_result.is_ok() {
            graphql_server_stop_result.unwrap();
        }

        // Be sure that the GraphQL server thread is gone
        if thread_handle.is_ok() {
            let _joined = thread_handle.unwrap().join();
        }
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

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager> {
        self.entity_behaviour_manager.clone()
    }

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager> {
        self.relation_behaviour_manager.clone()
    }

    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer> {
        self.graphql_server.clone()
    }
}
