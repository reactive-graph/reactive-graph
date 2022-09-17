use crate::model::FlowInstance;
use crate::model::ReactiveFlowInstance;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum FlowInstanceCreationError {
    Failed,
}

pub trait FlowInstanceManager: Send + Sync {
    /// Returns true, if an flow instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the flow instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveFlowInstance>>;

    /// Returns the flow instance with the given label or None.
    fn get_by_label(&self, label: String) -> Option<Arc<ReactiveFlowInstance>>;

    /// Creates a new reactive flow instance from the given flow instance descriptor.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// All reactive instances will be registered in the ReactiveEntityInstanceManager
    /// and the ReactiveRelationInstanceManager.
    fn create(&self, flow_instance: FlowInstance) -> Result<Arc<ReactiveFlowInstance>, FlowInstanceCreationError>;

    fn delete(&self, id: Uuid);
}
