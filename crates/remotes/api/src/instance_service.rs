use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_remotes_model::InstanceInfo;

#[injectable]
#[async_trait]
pub trait InstanceService: Send + Sync + Lifecycle {
    /// Returns the instance information.
    fn get_instance_info(&self) -> InstanceInfo;
}
