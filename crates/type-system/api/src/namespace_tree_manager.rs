use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_graph::Namespaces;

#[injectable]
#[async_trait]
pub trait NamespaceTreeManager: Send + Sync {
    /// Returns a flat tree of all paths.
    fn get_all(&self) -> Namespaces;
}
