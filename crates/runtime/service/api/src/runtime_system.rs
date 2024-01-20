use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;

use crate::ShutdownManager;

#[injectable]
#[async_trait]
pub trait RuntimeSystem: Lifecycle {
    fn get_shutdown_manager(&self) -> Arc<dyn ShutdownManager + Send + Sync>;
}
