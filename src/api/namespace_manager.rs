use std::collections::HashSet;

use async_trait::async_trait;

#[async_trait]
pub trait NamespaceManager: Send + Sync {
    /// Returns all namespaces.
    fn get_all(&self) -> HashSet<String>;
}
