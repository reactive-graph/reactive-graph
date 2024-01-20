use std::collections::HashSet;

use async_trait::async_trait;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait NamespaceManager: Send + Sync {
    /// Returns all namespaces.
    fn get_all(&self) -> HashSet<String>;
}
