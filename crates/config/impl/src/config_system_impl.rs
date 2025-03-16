use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_config_api::ConfigManager;
use reactive_graph_config_api::ConfigSystem;
use reactive_graph_lifecycle::Lifecycle;

#[derive(Component)]
pub struct ConfigSystemImpl {
    config_manager: Arc<dyn ConfigManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl ConfigSystem for ConfigSystemImpl {
    fn get_config_manager(&self) -> Arc<dyn ConfigManager + Send + Sync> {
        self.config_manager.clone()
    }
}

#[async_trait]
impl Lifecycle for ConfigSystemImpl {
    async fn init(&self) {
        self.config_manager.init().await;
    }

    async fn post_init(&self) {
        self.config_manager.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.config_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.config_manager.shutdown().await;
    }
}
