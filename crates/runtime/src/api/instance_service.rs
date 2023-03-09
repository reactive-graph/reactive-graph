use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::core_model::InstanceInfo;

#[async_trait]
pub trait InstanceService: Send + Sync + Lifecycle {
    /// Returns the instance information.
    fn get_instance_info(&self) -> InstanceInfo;
}
