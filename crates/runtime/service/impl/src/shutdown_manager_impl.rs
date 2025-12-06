use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::command::shutdown::shutdown_command;
use async_trait::async_trait;
use reactive_graph_command_model::reactive_graph::command::command::COMMAND;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_runtime_model::reactive_graph::runtime::shutdown::SHUTDOWN;
use reactive_graph_runtime_service_api::ShutdownManager;
use reactive_graph_runtime_service_api::UUID_SHUTDOWN;
use reactive_graph_type_system_api::EntityTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;

fn create_shutdown_state() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(false))
}

#[derive(Component)]
pub struct ShutdownManagerImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,

    #[component(default = "create_shutdown_state")]
    shutdown_state: Arc<AtomicBool>,
}

#[async_trait]
#[component_alias]
impl ShutdownManager for ShutdownManagerImpl {
    fn do_shutdown(&self) {
        self.shutdown_state.store(true, Ordering::Relaxed);
    }

    fn is_shutdown(&self) -> bool {
        self.shutdown_state.load(Ordering::Relaxed)
    }
}

#[async_trait]
impl Lifecycle for ShutdownManagerImpl {
    async fn init(&self) {
        // let shutdown_state = self.shutdown_state.clone();
        // let shutdown_command = shutdown_command(shutdown_state);
        // let _ = self.reactive_entity_manager.register_reactive_instance(shutdown_command.get_instance());
    }

    async fn post_init(&self) {
        let _ = self.entity_type_manager.add_component(&SHUTDOWN, &COMMAND);
        let shutdown_state = self.shutdown_state.clone();
        let shutdown_command = shutdown_command(shutdown_state);
        let _ = self.reactive_entity_manager.register_reactive_instance(shutdown_command.get_instance());
    }

    async fn shutdown(&self) {
        // Disconnect reactive streams of the shutdown handler
        if let Some(shutdown_handler) = self.reactive_entity_manager.get(UUID_SHUTDOWN) {
            shutdown_handler.remove_all_observers();
        }
    }
}
