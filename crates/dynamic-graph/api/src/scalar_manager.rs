use async_graphql::dynamic::Scalar;
use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait ScalarManager: Send + Sync + Lifecycle {
    /// Constructs the scalars.
    fn get_scalars(&self) -> Vec<Scalar>;
}
