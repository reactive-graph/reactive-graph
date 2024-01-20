use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_model_api::ReactivePropertyContainer;
use inexor_rgf_reactive_service_api::ReactiveEntityManager;
use inexor_rgf_runtime_service_api::ShutdownManager;
use inexor_rgf_runtime_service_api::UUID_SHUTDOWN;

use crate::command::shutdown::shutdown_command;

fn create_shutdown_state() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(false))
}

#[derive(Component)]
pub struct ShutdownManagerImpl {
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
