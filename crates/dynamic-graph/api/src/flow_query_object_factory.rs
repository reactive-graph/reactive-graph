use async_graphql::dynamic::Object;
use async_trait::async_trait;
use reactive_graph_graph::FlowType;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait FlowQueryObjectFactory: Send + Sync + Lifecycle {
    fn create_query_objects(&self) -> Vec<Object>;
    fn create_query_object(&self, flow_type: FlowType) -> Object;
}
