use crate::FlowTypeSystemRegistrator;
use async_trait::async_trait;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;
use std::sync::Arc;

#[injectable]
#[async_trait]
pub trait FlowSystem: Lifecycle {
    fn get_flow_type_system_registrator(&self) -> Arc<dyn FlowTypeSystemRegistrator + Send + Sync>;
}
