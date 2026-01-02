use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_service_api::ReactiveSystem;
use reactive_graph_type_system_api::TypeSystemSystem;

use crate::CommandManager;
use crate::CommandTypeSystemRegistrator;

#[injectable]
#[async_trait]
pub trait CommandSystem: Lifecycle {
    fn get_command_manager(&self) -> Arc<dyn CommandManager + Send + Sync>;

    fn get_command_type_system_registrator(&self) -> Arc<dyn CommandTypeSystemRegistrator + Send + Sync>;

    fn type_system_system(&self) -> Arc<dyn TypeSystemSystem + Send + Sync>;

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync>;
}
