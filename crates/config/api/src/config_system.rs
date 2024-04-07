use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;

use crate::ConfigManager;

#[injectable]
#[async_trait]
pub trait ConfigSystem: Lifecycle {
    fn get_config_manager(&self) -> Arc<dyn ConfigManager + Send + Sync>;
}
