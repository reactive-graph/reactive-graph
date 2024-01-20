use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_remotes_model::InstanceInfo;

#[injectable]
#[async_trait]
pub trait InstanceService: Send + Sync + Lifecycle {
    /// Returns the instance information.
    fn get_instance_info(&self) -> InstanceInfo;
}
