use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::model::{Flow, ReactiveFlow, ReactiveFlowConstructionError};
use crate::plugins::FlowProvider;

#[derive(Debug)]
pub enum ReactiveFlowCreationError {
    UuidTaken(Uuid),
    MissingWrapperInstance,
    // ReactiveEntityInstanceCreationError(ReactiveEntityInstanceCreationError),
    // ReactiveRelationInstanceCreationError(ReactiveRelationInstanceCreationError),
    ReactiveFlowConstructionError(ReactiveFlowConstructionError),
}

impl fmt::Display for ReactiveFlowCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ReactiveFlowCreationError::UuidTaken(id) => {
                write!(f, "The UUID {} has been already taken!", id)
            }
            ReactiveFlowCreationError::MissingWrapperInstance => {
                write!(f, "The created wrapper instance cannot be found")
            }
            // ReactiveFlowCreationError::ReactiveEntityInstanceCreationError(error) => write!(f, "Failed to create reactive entity instance: {}", error.to_string()),
            // ReactiveFlowCreationError::ReactiveRelationInstanceCreationError(error) => write!(f, "Failed to create reactive relation instance: {}", error.to_string())
            ReactiveFlowCreationError::ReactiveFlowConstructionError(error) => write!(f, "Failed to construct reactive flow: {}", error),
        }
    }
}

#[derive(Debug)]
pub struct ReactiveFlowImportError;

#[async_trait]
pub trait ReactiveFlowManager: Send + Sync + Lifecycle {
    /// Returns true, if an flow exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the flow with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveFlow>>;

    /// Returns all reactive flows.
    fn get_all(&self) -> Vec<Arc<ReactiveFlow>>;

    /// Creates a new reactive flow from the given flow description.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// All reactive instances will be registered in the ReactiveEntityInstanceManager
    /// and the ReactiveRelationInstanceManager.
    fn create(&self, flow: Flow) -> Result<Arc<ReactiveFlow>, ReactiveFlowCreationError>;

    /// Registers the given reactive flow and registers all of the reactive instances
    /// contained in the given reactive flow.
    fn register_flow_and_reactive_instances(&self, reactive_flow: Arc<ReactiveFlow>);

    /// Registers the given reactive flow. Does not register it's reactive instances except
    /// the wrapper entity.
    fn register_flow(&self, reactive_flow: Arc<ReactiveFlow>);

    /// The changes of the reactive flow with the given id will be written to graph database.
    // TODO: return result
    fn commit(&self, id: Uuid);

    fn delete(&self, id: Uuid);

    fn import(&self, path: String) -> Result<Arc<ReactiveFlow>, ReactiveFlowImportError>;

    // TODO: return result
    fn export(&self, id: Uuid, path: String);

    fn add_provider(&self, flow_provider: Arc<dyn FlowProvider>);
}
