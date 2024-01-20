use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_service_api::ReactiveSystem;
use inexor_rgf_type_system_api::TypeSystem;

use crate::CommandManager;
use crate::CommandTypeProvider;

#[injectable]
#[async_trait]
pub trait CommandSystem: Lifecycle {
    fn get_command_manager(&self) -> Arc<dyn CommandManager + Send + Sync>;

    fn get_command_type_provider(&self) -> Arc<dyn CommandTypeProvider + Send + Sync>;

    fn type_system(&self) -> Arc<dyn TypeSystem + Send + Sync>;

    fn reactive_system(&self) -> Arc<dyn ReactiveSystem + Send + Sync>;
}
