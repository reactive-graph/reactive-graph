use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_graph::NamespacedType;
use reactive_graph_graph::NamespacedTypes;

use crate::NamespacedTypeRegistrationError;

#[injectable]
#[async_trait]
pub trait NamespaceManager: Send + Sync {
    /// Registers the given namespaced type.
    fn register(&self, ty: NamespacedType) -> Result<NamespacedType, NamespacedTypeRegistrationError>;

    /// Returns all types.
    fn get_all(&self) -> NamespacedTypes;

    /// Returns true, if the given namespaced type exists.
    fn has(&self, ty: &NamespacedType) -> bool;

    /// Returns the count of namespaced types.
    fn count(&self) -> usize;

    /// Deletes the given namespaced type.
    fn delete(&self, ty: &NamespacedType) -> bool;
}
