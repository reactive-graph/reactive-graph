use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_graph::NamespacedType;
use reactive_graph_graph::NamespacedTypes;

use crate::NamespacedTypeRegistrationError;

/// Ensures every type in the type system has a unique namespace.
#[injectable]
#[async_trait]
pub trait NamespacedTypeManager: Send + Sync {
    /// Registers the given namespaced type.
    fn register(&self, ty: NamespacedType) -> Result<NamespacedType, NamespacedTypeRegistrationError>;

    /// Returns all types.
    fn get_all(&self) -> NamespacedTypes;

    /// Returns true, if the given namespaced type exists.
    fn has(&self, ty: &NamespacedType) -> bool;

    /// Returns the count of namespaced types.
    fn count(&self) -> usize;

    // TODO: Result<(), NamespacedTypeReplaceError>
    /// Remove the old namespaced type and registers the new namespaced type.
    fn replace(&self, old_ty: &NamespacedType, new_ty: NamespacedType) -> Result<bool, NamespacedTypeRegistrationError>;

    // TODO: Result<(), NamespacedTypeRemoveError>
    /// Deletes the given namespaced type.
    fn delete(&self, ty: &NamespacedType) -> bool;
}
