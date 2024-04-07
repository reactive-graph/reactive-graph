use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_runtime_service_api::RuntimeSystem;
use reactive_graph_runtime_service_api::ShutdownManager;

#[derive(Component)]
pub struct RuntimeSystemImpl {
    shutdown_manager: Arc<dyn ShutdownManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RuntimeSystem for RuntimeSystemImpl {
    fn get_shutdown_manager(&self) -> Arc<dyn ShutdownManager + Send + Sync> {
        self.shutdown_manager.clone()
    }
}

#[async_trait]
impl Lifecycle for RuntimeSystemImpl {
    async fn init(&self) {
        self.shutdown_manager.init().await;
    }

    async fn post_init(&self) {
        self.shutdown_manager.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.shutdown_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.shutdown_manager.shutdown().await;
    }
}
