use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait AppFactory: Send + Sync + Lifecycle {
    fn create_app(&self);
}
