use std::sync::Arc;

use async_trait::async_trait;
use inexor_rgf_config_api::ConfigSystem;
use springtime_di::component_alias;
use springtime_di::Component;

use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_remotes_api::InstanceService;
use inexor_rgf_remotes_api::RemotesManager;
use inexor_rgf_remotes_api::RemotesSystem;

#[derive(Component)]
pub struct RemotesSystemImpl {
    instance_service: Arc<dyn InstanceService + Send + Sync>,
    remotes_manager: Arc<dyn RemotesManager + Send + Sync>,

    config_system: Arc<dyn ConfigSystem + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RemotesSystem for RemotesSystemImpl {
    fn get_instance_service(&self) -> Arc<dyn InstanceService + Send + Sync> {
        self.instance_service.clone()
    }

    fn get_remotes_manager(&self) -> Arc<dyn RemotesManager + Send + Sync> {
        self.remotes_manager.clone()
    }

    fn config_system(&self) -> Arc<dyn ConfigSystem + Send + Sync> {
        self.config_system.clone()
    }
}

#[async_trait]
impl Lifecycle for RemotesSystemImpl {
    async fn init(&self) {
        self.instance_service.init().await;
        self.remotes_manager.init().await;
    }

    async fn post_init(&self) {
        self.instance_service.post_init().await;
        self.remotes_manager.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.remotes_manager.pre_shutdown().await;
        self.instance_service.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.remotes_manager.shutdown().await;
        self.instance_service.shutdown().await;
    }
}
