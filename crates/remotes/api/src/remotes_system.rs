use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_config_api::ConfigSystem;
use reactive_graph_lifecycle::Lifecycle;

use crate::InstanceService;
use crate::RemotesManager;

#[injectable]
#[async_trait]
pub trait RemotesSystem: Lifecycle {
    fn get_instance_service(&self) -> Arc<dyn InstanceService + Send + Sync>;
    fn get_remotes_manager(&self) -> Arc<dyn RemotesManager + Send + Sync>;

    fn config_system(&self) -> Arc<dyn ConfigSystem + Send + Sync>;
}
