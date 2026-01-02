use async_graphql::dynamic::Object;
use async_trait::async_trait;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait QueryObjectManager: Send + Sync + Lifecycle {
    /// Returns the query objects of the dynamic graph.
    fn get_query_objects(&self) -> Vec<Object>;
}
