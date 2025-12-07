use thiserror::Error;

use reactive_graph_graph::NamespacedType;

#[derive(Debug, Error)]
pub enum NamespacedTypeRegistrationError {
    #[error("Failed to register namespaced type {0} because it already exists!")]
    NamespacedTypeAlreadyExists(NamespacedType),
}
