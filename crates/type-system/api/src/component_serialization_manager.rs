use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_lifecycle::Lifecycle;

use crate::ComponentSerializeError;

#[injectable]
#[async_trait]
pub trait ComponentSerializationManager: Send + Sync + Lifecycle {
    async fn serialize(&self, ty: &ComponentTypeId) -> Result<String, ComponentSerializeError>;
}
