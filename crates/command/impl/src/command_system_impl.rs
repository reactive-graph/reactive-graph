use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use inexor_rgf_command_api::CommandManager;
use inexor_rgf_command_api::CommandSystem;
use inexor_rgf_command_api::CommandTypeProvider;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_service_api::ReactiveSystem;
use inexor_rgf_type_system_api::TypeSystem;

#[derive(Component)]
pub struct CommandSystemImpl {
    command_manager: Arc<dyn CommandManager + Send + Sync>,
    command_type_provider: Arc<dyn CommandTypeProvider + Send + Sync>,

    type_system: Arc<dyn TypeSystem + Send + Sync>,
    reactive_system: Arc<dyn ReactiveSystem + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl CommandSystem for CommandSystemImpl {
    fn get_command_manager(&self) -> Arc<dyn CommandManager + Send + Sync> {
        self.command_manager.clone()
    }

    fn get_command_type_provider(&self) -> Arc<dyn CommandTypeProvider + Send + Sync> {
        self.command_type_provider.clone()
    }

    fn type_system(&self) -> Arc<dyn TypeSystem + Send + Sync> {
        self.type_system.clone()
    }

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync> {
        self.reactive_system.clone()
    }
}

#[async_trait]
impl Lifecycle for CommandSystemImpl {
    async fn init(&self) {
        self.command_type_provider.init().await;
        self.command_manager.init().await;
    }

    async fn post_init(&self) {
        self.command_type_provider.post_init().await;
        self.command_manager.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.command_manager.pre_shutdown().await;
        self.command_type_provider.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.command_manager.shutdown().await;
        self.command_type_provider.shutdown().await;
    }
}
