use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::api::ReactiveEntityManager;
use crate::api::ShutdownManager;
use crate::api::UUID_SHUTDOWN;
use crate::commands::shutdown::shutdown_command;
use crate::di::*;
use crate::reactive::ReactivePropertyContainer;

#[wrapper]
pub struct ShutdownStateContainer(Arc<RwLock<bool>>);

#[provides]
fn create_shutdown_state() -> ShutdownStateContainer {
    ShutdownStateContainer(Arc::new(RwLock::new(false)))
}

#[component]
pub struct ShutdownManagerImpl {
    reactive_entity_manager: Wrc<dyn ReactiveEntityManager>,

    shutdown_state: ShutdownStateContainer,
}

#[async_trait]
#[provides]
impl ShutdownManager for ShutdownManagerImpl {
    fn do_shutdown(&self) {
        let mut guard = self.shutdown_state.0.write().unwrap();
        *guard = true;
    }

    fn is_shutdown(&self) -> bool {
        *self.shutdown_state.0.read().unwrap().deref()
    }
}

#[async_trait]
impl Lifecycle for ShutdownManagerImpl {
    async fn init(&self) {
        let shutdown_state = self.shutdown_state.0.clone();
        let shutdown_command = shutdown_command(shutdown_state);
        let _ = self
            .reactive_entity_manager
            .register_reactive_instance(shutdown_command.get_instance());
    }

    async fn shutdown(&self) {
        // Disconnect reactive streams of the shutdown handler
        if let Some(shutdown_handler) = self.reactive_entity_manager.get(UUID_SHUTDOWN) {
            shutdown_handler.remove_all_observers();
        }
    }
}
