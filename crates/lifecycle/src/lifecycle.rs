/// Dual layer runtime lifecycle for initialization and shutdown of services
use async_trait::async_trait;

#[async_trait]
pub trait Lifecycle: Sync {
    /// Called at initialization
    async fn init(&self) {}

    /// Called after initialization
    async fn post_init(&self) {}

    /// Called before shutdown
    async fn pre_shutdown(&self) {}

    /// Called for shutdown
    async fn shutdown(&self) {}
}
