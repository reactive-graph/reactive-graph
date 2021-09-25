use crate::model::{Flow, ReactiveFlow};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum FlowCreationError {
    Failed,
}

pub trait FlowManager: Send + Sync {
    /// Returns true, if an flow exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the flow with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveFlow>>;

    /// Creates a new reactive flow from the given flow description.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// All reactive instances will be registered in the ReactiveEntityInstanceManager
    /// and the ReactiveRelationInstanceManager.
    fn create(&self, flow: Flow) -> Result<Arc<ReactiveFlow>, FlowCreationError>;

    fn delete(&self, id: Uuid);
}
