use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait FlowTypeSystemRegistrator: Send + Sync + Lifecycle {}
