use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;

use async_trait::async_trait;
use log::debug;
use log::info;
use springtime_di::Component;
use springtime_di::component_alias;
use tokio::time::error::Elapsed;

use reactive_graph_behaviour_service_api::BehaviourSystem;
use reactive_graph_behaviour_service_api::EntityBehaviourManager;
use reactive_graph_behaviour_service_api::EntityBehaviourRegistry;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourManager;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourRegistry;
use reactive_graph_behaviour_service_api::RelationBehaviourManager;
use reactive_graph_behaviour_service_api::RelationBehaviourRegistry;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourManager;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourRegistry;
use reactive_graph_command_api::CommandManager;
use reactive_graph_command_api::CommandSystem;
use reactive_graph_command_api::CommandTypeSystemRegistrator;
use reactive_graph_command_impl::CommandSystemImpl;
use reactive_graph_config_api::ConfigManager;
use reactive_graph_config_api::ConfigSystem;
use reactive_graph_config_impl::ConfigSystemImpl;
use reactive_graph_dynamic_graph_api::DynamicGraphQueryService;
use reactive_graph_dynamic_graph_api::DynamicGraphSchemaBuilder;
use reactive_graph_dynamic_graph_api::DynamicGraphSchemaManager;
use reactive_graph_dynamic_graph_api::DynamicGraphSystem;
use reactive_graph_dynamic_graph_api::SchemaBuilderContextManager;
use reactive_graph_dynamic_graph_api::SchemaBuilderManager;
use reactive_graph_flow_api::FlowSystem;
use reactive_graph_flow_api::FlowTypeSystemRegistrator;
use reactive_graph_flow_impl::flow_system_impl::FlowSystemImpl;
use reactive_graph_graphql_api::GraphQLQueryService;
use reactive_graph_graphql_api::GraphQLSchemaManager;
use reactive_graph_graphql_api::GraphQLSystem;
use reactive_graph_instance_system_api::EntityInstanceImportExportManager;
use reactive_graph_instance_system_api::InstanceSystem;
use reactive_graph_instance_system_api::RelationInstanceImportExportManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_graphql_api::PluginGraphQLSystem;
use reactive_graph_plugin_graphql_api::PluginQueryService;
use reactive_graph_plugin_graphql_api::PluginSchemaManager;
use reactive_graph_plugin_service_api::PluginContainerManager;
use reactive_graph_plugin_service_api::PluginContextFactory;
use reactive_graph_plugin_service_api::PluginRepositoryManager;
use reactive_graph_plugin_service_api::PluginResolver;
use reactive_graph_plugin_service_api::PluginSystem;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_reactive_service_api::ReactiveInstanceEventManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_reactive_service_api::ReactiveSystem;
use reactive_graph_remotes_api::InstanceService;
use reactive_graph_remotes_api::RemotesManager;
use reactive_graph_remotes_api::RemotesSystem;
use reactive_graph_remotes_impl::RemotesSystemImpl;
use reactive_graph_remotes_model::InstanceAddress;
use reactive_graph_runtime_api::Runtime;
use reactive_graph_runtime_graphql_api::RuntimeGraphQLSystem;
use reactive_graph_runtime_graphql_api::RuntimeQueryService;
use reactive_graph_runtime_graphql_api::RuntimeSchemaManager;
use reactive_graph_runtime_service_api::RuntimeSystem;
use reactive_graph_runtime_service_api::ShutdownManager;
use reactive_graph_runtime_web_api::GraphQLServer;
use reactive_graph_runtime_web_api::WebResourceManager;
use reactive_graph_runtime_web_api::WebSystem;
use reactive_graph_type_system_api::ComponentImportExportManager;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeImportExportManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeImportExportManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::NamespaceTreeManager;
use reactive_graph_type_system_api::NamespacedTypeManager;
use reactive_graph_type_system_api::RelationTypeImportExportManager;
use reactive_graph_type_system_api::RelationTypeManager;
use reactive_graph_type_system_api::TypeSystemEventManager;
use reactive_graph_type_system_api::TypeSystemProviderRegistry;
use reactive_graph_type_system_api::TypeSystemSystem;

pub struct RunningState(Arc<AtomicBool>);

fn create_running_state() -> RunningState {
    RunningState(Arc::new(AtomicBool::new(false)))
}

#[derive(Component)]
pub struct RuntimeImpl {
    #[component(default = "create_running_state")]
    running: RunningState,
    type_system_system: Arc<dyn TypeSystemSystem + Send + Sync>,
    reactive_system: Arc<dyn ReactiveSystem + Send + Sync>,
    behaviour_system: Arc<dyn BehaviourSystem + Send + Sync>,
    instance_system: Arc<dyn InstanceSystem + Send + Sync>,
    flow_system: Arc<FlowSystemImpl>,
    command_system: Arc<CommandSystemImpl>,
    runtime_system: Arc<dyn RuntimeSystem + Send + Sync>,
    remotes_system: Arc<RemotesSystemImpl>,
    config_system: Arc<ConfigSystemImpl>,
    graphql_system: Arc<dyn GraphQLSystem + Send + Sync>,
    dynamic_graph_system: Arc<dyn DynamicGraphSystem + Send + Sync>,
    runtime_graphql_system: Arc<dyn RuntimeGraphQLSystem + Send + Sync>,
    plugin_graphql_system: Arc<dyn PluginGraphQLSystem + Send + Sync>,
    plugin_system: Arc<dyn PluginSystem + Send + Sync>,
    web_system: Arc<dyn WebSystem + Send + Sync>,
}

impl RuntimeImpl {
    async fn wait_for_started_internal(&self) {
        // TODO: Add upper bound timeout (for example 30 sec)
        while !self.is_running() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

#[async_trait]
#[component_alias]
impl Runtime for RuntimeImpl {
    async fn config(&self) {
        self.config_system.init().await;
    }

    async fn run(&self) {
        // Signal handling
        let terminate = Arc::new(AtomicBool::new(false));
        // This channel allows the main thread to stop the GraphQL server thread
        let (graphql_server_stop_sender, graphql_server_stop_receiver) = crossbeam::channel::unbounded::<()>();
        // This channel allows the GraphQL server thread to tell the main thread that it has been finished
        let (graphql_server_stopped_sender, graphql_server_stopped_receiver) = crossbeam::channel::unbounded::<()>();
        // Clone GraphQL server and move the reference into the GraphQL server thread
        let graphql_server = self.web_system.get_graphql_server();
        // GraphQL server thread: Create a new thread for the GraphQL server
        // TODO: Use tokio task Builder to name the task
        // tokio::task::Builder::new().name("rg_server").spawn()
        let graphql_server_handle = tokio::spawn(async move {
            // Run the GraphQL server
            info!("Run the GraphQL server.");
            graphql_server.serve(graphql_server_stop_receiver).await;
            debug!("Successfully stopped GraphQL Server.");
            // Tell the main thread, that the GraphQL server thread has finished
            let _result = graphql_server_stopped_sender.send(());
        });

        {
            self.running.0.store(true, Ordering::Relaxed);
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
                if self.runtime_system.get_shutdown_manager().is_shutdown() {
                    stopping = true;
                }
            }
        } // Drop "running"

        // Stop GraphQL server thread, if it is still running
        debug!("Stopping the GraphQL server thread");
        let _graphql_server_stop_result = graphql_server_stop_sender.send_timeout((), Duration::from_millis(100));

        // Be sure that the GraphQL server thread is gone
        let _ = graphql_server_handle.await;
        info!("Bye.");

        // Ensure the running state is now set to false even if the loop was terminated
        // externally because the running state is checked from outside.
        {
            self.running.0.store(false, Ordering::Relaxed);
        }
    }

    fn stop(&self) {
        {
            self.running.0.store(false, Ordering::Relaxed);
        }
    }

    fn is_running(&self) -> bool {
        self.running.0.load(Ordering::Relaxed)
        // *self.running.0.read().unwrap().deref()
    }

    async fn wait_for_started(&self, timeout_duration: Duration) -> Result<(), Elapsed> {
        tokio::time::timeout(timeout_duration, self.wait_for_started_internal()).await
    }

    async fn wait_for_stopped(&self) {
        while self.is_running() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn wait_for_stopped_with_timeout(&self, timeout_duration: Duration) -> Result<(), Elapsed> {
        tokio::time::timeout(timeout_duration, self.wait_for_stopped()).await
    }

    fn address(&self) -> InstanceAddress {
        self.remotes_system.get_instance_service().get_instance_info().address()
    }
}

impl TypeSystemSystem for RuntimeImpl {
    fn get_component_manager(&self) -> Arc<dyn ComponentManager + Send + Sync> {
        self.type_system_system.get_component_manager()
    }

    fn get_component_import_export_manager(&self) -> Arc<dyn ComponentImportExportManager + Send + Sync> {
        self.type_system_system.get_component_import_export_manager()
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager + Send + Sync> {
        self.type_system_system.get_entity_type_manager()
    }

    fn get_entity_type_import_export_manager(&self) -> Arc<dyn EntityTypeImportExportManager + Send + Sync> {
        self.type_system_system.get_entity_type_import_export_manager()
    }

    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager + Send + Sync> {
        self.type_system_system.get_flow_type_manager()
    }

    fn get_flow_type_import_export_manager(&self) -> Arc<dyn FlowTypeImportExportManager + Send + Sync> {
        self.type_system_system.get_flow_type_import_export_manager()
    }

    fn get_namespace_tree_manager(&self) -> Arc<dyn NamespaceTreeManager + Send + Sync> {
        self.type_system_system.get_namespace_tree_manager()
    }

    fn get_namespaced_type_manager(&self) -> Arc<dyn NamespacedTypeManager + Send + Sync> {
        self.type_system_system.get_namespaced_type_manager()
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager + Send + Sync> {
        self.type_system_system.get_relation_type_manager()
    }

    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager + Send + Sync> {
        self.type_system_system.get_relation_type_import_export_manager()
    }

    fn get_type_system_event_manager(&self) -> Arc<dyn TypeSystemEventManager + Send + Sync> {
        self.type_system_system.get_type_system_event_manager()
    }

    fn get_type_system_provider_registry(&self) -> Arc<dyn TypeSystemProviderRegistry + Send + Sync> {
        self.type_system_system.get_type_system_provider_registry()
    }
}

impl ReactiveSystem for RuntimeImpl {
    fn get_reactive_entity_manager(&self) -> Arc<dyn ReactiveEntityManager + Send + Sync> {
        self.reactive_system.get_reactive_entity_manager()
    }

    fn get_reactive_flow_manager(&self) -> Arc<dyn ReactiveFlowManager + Send + Sync> {
        self.reactive_system.get_reactive_flow_manager()
    }

    fn get_reactive_relation_manager(&self) -> Arc<dyn ReactiveRelationManager + Send + Sync> {
        self.reactive_system.get_reactive_relation_manager()
    }

    fn get_reactive_instance_event_manager(&self) -> Arc<dyn ReactiveInstanceEventManager + Send + Sync> {
        self.reactive_system.get_reactive_instance_event_manager()
    }

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync> {
        self.reactive_system.type_system_system()
    }

    fn behaviour_system(&self) -> Arc<dyn BehaviourSystem + Send + Sync> {
        self.reactive_system.behaviour_system()
    }
}

impl BehaviourSystem for RuntimeImpl {
    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager + Send + Sync> {
        self.behaviour_system.get_entity_behaviour_manager()
    }

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry + Send + Sync> {
        self.behaviour_system.get_entity_behaviour_registry()
    }

    fn get_entity_component_behaviour_manager(&self) -> Arc<dyn EntityComponentBehaviourManager + Send + Sync> {
        self.behaviour_system.get_entity_component_behaviour_manager()
    }

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry + Send + Sync> {
        self.behaviour_system.get_entity_component_behaviour_registry()
    }

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager + Send + Sync> {
        self.behaviour_system.get_relation_behaviour_manager()
    }

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry + Send + Sync> {
        self.behaviour_system.get_relation_behaviour_registry()
    }

    fn get_relation_component_behaviour_manager(&self) -> Arc<dyn RelationComponentBehaviourManager + Send + Sync> {
        self.behaviour_system.get_relation_component_behaviour_manager()
    }

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry + Send + Sync> {
        self.behaviour_system.get_relation_component_behaviour_registry()
    }

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync> {
        self.behaviour_system.type_system_system()
    }
}

impl InstanceSystem for RuntimeImpl {
    fn get_entity_instance_import_export_manager(&self) -> Arc<dyn EntityInstanceImportExportManager + Send + Sync> {
        self.instance_system.get_entity_instance_import_export_manager()
    }

    fn get_relation_instance_import_export_manager(&self) -> Arc<dyn RelationInstanceImportExportManager + Send + Sync> {
        self.instance_system.get_relation_instance_import_export_manager()
    }

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync> {
        self.reactive_system.clone()
    }
}

impl FlowSystem for RuntimeImpl {
    fn get_flow_type_system_registrator(&self) -> Arc<dyn FlowTypeSystemRegistrator + Send + Sync> {
        self.flow_system.get_flow_type_system_registrator()
    }
}

impl CommandSystem for RuntimeImpl {
    fn get_command_manager(&self) -> Arc<dyn CommandManager + Send + Sync> {
        self.command_system.get_command_manager()
    }

    fn get_command_type_system_registrator(&self) -> Arc<dyn CommandTypeSystemRegistrator + Send + Sync> {
        self.command_system.get_command_type_system_registrator()
    }

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync> {
        self.command_system.type_system_system()
    }

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync> {
        self.command_system.reactive_system()
    }
}

impl ConfigSystem for RuntimeImpl {
    fn get_config_manager(&self) -> Arc<dyn ConfigManager + Send + Sync> {
        self.config_system.get_config_manager()
    }
}

impl RemotesSystem for RuntimeImpl {
    fn get_instance_service(&self) -> Arc<dyn InstanceService + Send + Sync> {
        self.remotes_system.get_instance_service()
    }

    fn get_remotes_manager(&self) -> Arc<dyn RemotesManager + Send + Sync> {
        self.remotes_system.get_remotes_manager()
    }

    fn config_system(&self) -> Arc<dyn ConfigSystem + Send + Sync> {
        self.remotes_system.config_system()
    }
}

impl RuntimeSystem for RuntimeImpl {
    fn get_shutdown_manager(&self) -> Arc<dyn ShutdownManager + Send + Sync> {
        self.runtime_system.get_shutdown_manager()
    }
}

impl WebSystem for RuntimeImpl {
    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer + Send + Sync> {
        self.web_system.get_graphql_server()
    }

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager + Send + Sync> {
        self.web_system.get_web_resource_manager()
    }

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync> {
        self.web_system.type_system_system()
    }

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync> {
        self.web_system.reactive_system()
    }

    fn config_system(&self) -> Arc<dyn ConfigSystem + Send + Sync> {
        self.web_system.config_system()
    }

    fn runtime_graphql_system(&self) -> Arc<dyn RuntimeGraphQLSystem + Send + Sync> {
        self.web_system.runtime_graphql_system()
    }

    fn plugin_graphql_system(&self) -> Arc<dyn PluginGraphQLSystem + Send + Sync> {
        self.web_system.plugin_graphql_system()
    }

    fn dynamic_graph_system(&self) -> Arc<dyn DynamicGraphSystem + Send + Sync> {
        self.web_system.dynamic_graph_system()
    }

    fn graphql_system(&self) -> Arc<dyn GraphQLSystem + Send + Sync> {
        self.web_system.graphql_system()
    }
}

impl GraphQLSystem for RuntimeImpl {
    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService + Send + Sync> {
        self.graphql_system.get_graphql_query_service()
    }

    fn get_graphql_schema_manager(&self) -> Arc<dyn GraphQLSchemaManager + Send + Sync> {
        self.graphql_system.get_graphql_schema_manager()
    }
}

impl DynamicGraphSystem for RuntimeImpl {
    fn get_dynamic_graph_query_service(&self) -> Arc<dyn DynamicGraphQueryService + Send + Sync> {
        self.dynamic_graph_system.get_dynamic_graph_query_service()
    }

    fn get_dynamic_graph_schema_builder(&self) -> Arc<dyn DynamicGraphSchemaBuilder + Send + Sync> {
        self.dynamic_graph_system.get_dynamic_graph_schema_builder()
    }

    fn get_dynamic_graph_schema_manager(&self) -> Arc<dyn DynamicGraphSchemaManager + Send + Sync> {
        self.dynamic_graph_system.get_dynamic_graph_schema_manager()
    }

    fn get_schema_builder_context_manager(&self) -> Arc<dyn SchemaBuilderContextManager + Send + Sync> {
        self.dynamic_graph_system.get_schema_builder_context_manager()
    }

    fn get_schema_builder_manager(&self) -> Arc<dyn SchemaBuilderManager + Send + Sync> {
        self.dynamic_graph_system.get_schema_builder_manager()
    }

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync> {
        self.dynamic_graph_system.type_system_system()
    }

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync> {
        self.dynamic_graph_system.reactive_system()
    }
}

impl RuntimeGraphQLSystem for RuntimeImpl {
    fn get_runtime_query_service(&self) -> Arc<dyn RuntimeQueryService + Send + Sync> {
        self.runtime_graphql_system.get_runtime_query_service()
    }

    fn get_runtime_schema_manager(&self) -> Arc<dyn RuntimeSchemaManager + Send + Sync> {
        self.runtime_graphql_system.get_runtime_schema_manager()
    }
}

impl PluginGraphQLSystem for RuntimeImpl {
    fn get_plugin_query_service(&self) -> Arc<dyn PluginQueryService + Send + Sync> {
        self.plugin_graphql_system.get_plugin_query_service()
    }

    fn get_plugin_schema_manager(&self) -> Arc<dyn PluginSchemaManager + Send + Sync> {
        self.plugin_graphql_system.get_plugin_schema_manager()
    }
}

impl PluginSystem for RuntimeImpl {
    fn get_plugin_context_factory(&self) -> Arc<dyn PluginContextFactory + Send + Sync> {
        self.plugin_system.get_plugin_context_factory()
    }

    fn get_plugin_container_manager(&self) -> Arc<dyn PluginContainerManager + Send + Sync> {
        self.plugin_system.get_plugin_container_manager()
    }

    fn get_plugin_repository_manager(&self) -> Arc<dyn PluginRepositoryManager + Send + Sync> {
        self.plugin_system.get_plugin_repository_manager()
    }

    fn get_plugin_resolver(&self) -> Arc<dyn PluginResolver + Send + Sync> {
        self.plugin_system.get_plugin_resolver()
    }
}

#[async_trait]
impl Lifecycle for RuntimeImpl {
    async fn init(&self) {
        // Order matters
        self.type_system_system.init().await;
        self.reactive_system.init().await;
        self.behaviour_system.init().await;
        self.instance_system.init().await;
        self.flow_system.init().await;
        //
        self.runtime_system.init().await;
        self.command_system.init().await;
        // self.shutdown_manager.init().await;
        self.remotes_system.init().await;
        //
        self.graphql_system.init().await;
        self.dynamic_graph_system.init().await;
        //
        self.web_system.init().await;
        // self.web_resource_manager.init().await;
        // self.graphql_server.init().await;
        //
        self.plugin_system.init().await;
    }

    async fn post_init(&self) {
        // Order matters
        self.type_system_system.post_init().await;
        self.reactive_system.post_init().await;
        self.behaviour_system.post_init().await;
        self.instance_system.post_init().await;
        self.flow_system.post_init().await;
        //
        self.runtime_system.post_init().await;
        self.command_system.post_init().await;
        // self.shutdown_manager.post_init().await;
        self.remotes_system.post_init().await;
        //
        self.graphql_system.post_init().await;
        self.dynamic_graph_system.post_init().await;
        //
        self.web_system.post_init().await;
        // self.web_resource_manager.post_init().await;
        // self.graphql_server.post_init().await;
        //
        self.plugin_system.post_init().await;
    }

    async fn pre_shutdown(&self) {
        // Reverse order matters
        self.plugin_system.pre_shutdown().await;
        //
        // self.graphql_server.pre_shutdown().await;
        // self.web_resource_manager.pre_shutdown().await;
        self.web_system.pre_shutdown().await;
        //
        self.dynamic_graph_system.pre_shutdown().await;
        self.graphql_system.pre_shutdown().await;
        //
        self.remotes_system.pre_shutdown().await;
        // self.shutdown_manager.pre_shutdown().await;
        self.command_system.pre_shutdown().await;
        self.runtime_system.pre_shutdown().await;
        //
        self.flow_system.pre_shutdown().await;
        self.instance_system.pre_shutdown().await;
        self.behaviour_system.pre_shutdown().await;
        self.reactive_system.pre_shutdown().await;
        self.type_system_system.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        // Reverse order matters
        self.plugin_system.shutdown().await;
        //
        // self.graphql_server.shutdown().await;
        // self.web_resource_manager.shutdown().await;
        self.web_system.shutdown().await;
        //
        self.dynamic_graph_system.shutdown().await;
        self.graphql_system.shutdown().await;
        //
        self.remotes_system.shutdown().await;
        // self.shutdown_manager.shutdown().await;
        self.command_system.shutdown().await;
        self.runtime_system.shutdown().await;
        //
        self.flow_system.shutdown().await;
        self.instance_system.shutdown().await;
        self.behaviour_system.shutdown().await;
        self.reactive_system.shutdown().await;
        self.type_system_system.shutdown().await;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use log::LevelFilter;
    use log4rs::Config;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::Appender;
    use log4rs::config::Root;

    use crate::get_runtime;

    /// This starts the runtime in an async environment.
    ///
    /// The runtime will be started including GraphQL server and fully
    /// initialized. After 2 seconds the runtime will be stopped.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_run() {
        let stdout = ConsoleAppender::builder().build();
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
            .expect("Failed to create logger");
        if let Err(error) = log4rs::init_config(config) {
            eprintln!("Failed to configure logger: {}", error);
        }
        let rt = get_runtime();
        let runtime = rt.clone();
        tokio::spawn(async move {
            let runtime = runtime;
            runtime.init().await;
            runtime.post_init().await;
            runtime.run().await;
            runtime.pre_shutdown().await;
            runtime.shutdown().await;
        });
        tokio::time::sleep(Duration::from_secs(2)).await;
        rt.stop();
    }
}
