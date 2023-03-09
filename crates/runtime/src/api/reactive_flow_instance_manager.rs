use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::model::EntityTypeId;
use crate::model::FlowInstance;
use crate::model::FlowTypeId;
use crate::model::ReactiveFlowInstance;
use crate::model::ReactiveFlowInstanceConstructionError;
use crate::model::RelationTypeId;
use crate::model::TypeDefinitionGetter;
use crate::plugins::FlowInstanceProvider;

#[derive(Debug)]
pub enum ReactiveFlowInstanceCreationError {
    UuidTaken(Uuid),
    MissingWrapperInstance,
    // ReactiveEntityInstanceCreationError(ReactiveEntityInstanceCreationError),
    // ReactiveRelationInstanceCreationError(ReactiveRelationInstanceCreationError),
    ReactiveFlowInstanceConstructionError(ReactiveFlowInstanceConstructionError),
    MissingVariable(String),
    FlowTypeDoesntExist(FlowTypeId),
    EntityTypeDoesntExist(EntityTypeId),
    RelationTypeDoesntExist(RelationTypeId),
    InvalidOutboundId(Uuid),
    InvalidInboundId(Uuid),
}

impl fmt::Display for ReactiveFlowInstanceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ReactiveFlowInstanceCreationError::UuidTaken(id) => {
                write!(f, "The UUID {} has been already taken!", id)
            }
            ReactiveFlowInstanceCreationError::MissingWrapperInstance => {
                write!(f, "The created wrapper instance cannot be found")
            }
            // ReactiveFlowInstanceCreationError::ReactiveEntityInstanceCreationError(error) => write!(f, "Failed to create reactive entity instance: {}", error.to_string()),
            // ReactiveFlowInstanceCreationError::ReactiveRelationInstanceCreationError(error) => write!(f, "Failed to create reactive relation instance: {}", error.to_string())
            ReactiveFlowInstanceCreationError::ReactiveFlowInstanceConstructionError(error) => write!(f, "Failed to construct reactive flow: {}", error),
            ReactiveFlowInstanceCreationError::MissingVariable(variable_name) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type requires variable {} which wasn't provided",
                    variable_name
                )
            }
            ReactiveFlowInstanceCreationError::FlowTypeDoesntExist(flow_ty) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type {} doesn't exist",
                    flow_ty.type_definition().to_string()
                )
            }
            ReactiveFlowInstanceCreationError::EntityTypeDoesntExist(entity_ty) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type contains an entity instance of type {} which doesn't exist",
                    entity_ty.type_definition().to_string()
                )
            }
            ReactiveFlowInstanceCreationError::RelationTypeDoesntExist(relation_ty) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type contains a relation instance of type {} which doesn't exist",
                    relation_ty.type_definition().to_string()
                )
            }
            ReactiveFlowInstanceCreationError::InvalidOutboundId(id) => {
                write!(f, "Failed to construct reactive flow instance: Flow type provides relation instance which outbound refers to a entity instance with id {} which doesn't exist", id)
            }
            ReactiveFlowInstanceCreationError::InvalidInboundId(id) => {
                write!(f, "Failed to construct reactive flow instance: Flow type provides relation instance which inbound refers to entity instance with id {} which doesn't exist", id)
            }
        }
    }
}

impl From<ReactiveFlowInstanceCreationError> for String {
    fn from(e: ReactiveFlowInstanceCreationError) -> Self {
        format!("{}", e)
    }
}

#[derive(Debug)]
pub struct ReactiveFlowInstanceImportError;

#[async_trait]
pub trait ReactiveFlowInstanceManager: Send + Sync + Lifecycle {
    /// Returns true, if an flow instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the flow instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveFlowInstance>>;

    /// Returns the flow instance that matches the given label or None.
    fn get_by_label(&self, label: &str) -> Option<Arc<ReactiveFlowInstance>>;

    /// Returns all reactive flow instances.
    fn get_all(&self) -> Vec<Arc<ReactiveFlowInstance>>;

    /// Returns the count of registered reactive flow instances.
    fn count_flow_instances(&self) -> usize;

    /// Creates a new reactive flow instance from the given flow instance descriptor.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// All reactive instances will be registered in the ReactiveEntityInstanceManager
    /// and the ReactiveRelationInstanceManager.
    fn create(&self, flow_instance: FlowInstance) -> Result<Arc<ReactiveFlowInstance>, ReactiveFlowInstanceCreationError>;

    /// Create a new reactive flow instance from the flow type by the given name.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// The properties are assigned to the wrapper entity instance.
    ///
    /// The variables will replace the property value.
    ///
    /// All reactive instances will be registered in the ReactiveEntityInstanceManager
    /// and the ReactiveRelationInstanceManager.
    fn create_from_type(
        &self,
        ty: &FlowTypeId,
        variables: HashMap<String, Value>,
        properties: HashMap<String, Value>,
    ) -> Result<Arc<ReactiveFlowInstance>, ReactiveFlowInstanceCreationError>;

    /// Registers the given reactive flow instance and registers all of the reactive instances
    /// contained in the given reactive flow instance.
    fn register_flow_instance_and_reactive_instances(&self, reactive_flow_instance: Arc<ReactiveFlowInstance>);

    /// Registers the given reactive flow instance. Does not register it's reactive instances except
    /// the wrapper entity.
    fn register_flow_instance(&self, reactive_flow_instance: Arc<ReactiveFlowInstance>);

    /// The changes of the reactive flow instance with the given id will be written to graph database.
    // TODO: return result
    fn commit(&self, id: Uuid);

    /// Deletes the flow instance with the given id.
    fn delete(&self, id: Uuid);

    fn import(&self, path: &str) -> Result<Arc<ReactiveFlowInstance>, ReactiveFlowInstanceImportError>;

    // TODO: return result
    fn export(&self, id: Uuid, path: &str);

    /// Registers a flow instance provider.
    fn add_provider(&self, id: Uuid, flow_instance_provider: Arc<dyn FlowInstanceProvider>);

    /// Unregisters a flow instance provider.
    fn remove_provider(&self, id: &Uuid);
}
