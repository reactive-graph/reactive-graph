use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::model::FlowInstance;
use crate::model::FlowTypeType;
use crate::model::ReactiveFlowInstance;

#[derive(Debug)]
pub enum FlowInstanceManagerError {
    InitializationError,
}

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
    fn get_by_label(&self, label: &str) -> Option<Arc<ReactiveFlowInstance>>;

    /// Creates a new reactive flow instance from the given flow instance descriptor.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// All reactive instances will be registered in the ReactiveEntityInstanceManager
    /// and the ReactiveRelationInstanceManager.
    fn create(&self, flow_instance: FlowInstance) -> Result<Arc<ReactiveFlowInstance>, FlowInstanceCreationError>;

    /// Create a new reactive flow instance from the flow type by the given name.
    ///
    /// It's possible to individualize the flow instance with templating using the given variables.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// All reactive instances will be registered in the ReactiveEntityInstanceManager
    /// and the ReactiveRelationInstanceManager.
    fn create_from_type(
        &self,
        ty: &FlowTypeType,
        variables: HashMap<String, Value>,
        properties: HashMap<String, Value>,
    ) -> Result<Arc<ReactiveFlowInstance>, FlowInstanceCreationError>;

    /// Deletes the flow instance with the given id.
    fn delete(&self, id: Uuid);
}
