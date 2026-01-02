use async_graphql::dynamic::Object;
use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait MutationObjectManager: Send + Sync + Lifecycle {
    /// Returns the mutation objects of the dynamic graph.
    fn get_mutation_objects(&self) -> Vec<Object>;
}
