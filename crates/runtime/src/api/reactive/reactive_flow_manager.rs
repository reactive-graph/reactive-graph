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
use crate::reactive::ReactiveFlow;
use crate::reactive::ReactiveFlowConstructionError;
use crate::model::RelationTypeId;
use crate::model::TypeDefinitionGetter;
use crate::plugins::FlowInstanceProvider;

#[derive(Debug)]
pub enum ReactiveFlowCreationError {
    UuidTaken(Uuid),
    MissingWrapperInstance,
    // ReactiveEntityCreationError(ReactiveEntityCreationError),
    // ReactiveRelationCreationError(ReactiveRelationCreationError),
    ReactiveFlowConstructionError(ReactiveFlowConstructionError),
    MissingVariable(String),
    FlowTypeDoesntExist(FlowTypeId),
    EntityTypeDoesntExist(EntityTypeId),
    RelationTypeDoesntExist(RelationTypeId),
    InvalidOutboundId(Uuid),
    InvalidInboundId(Uuid),
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
            // ReactiveFlowCreationError::ReactiveEntityCreationError(error) => write!(f, "Failed to create reactive entity instance: {}", error.to_string()),
            // ReactiveFlowCreationError::ReactiveRelationCreationError(error) => write!(f, "Failed to create reactive relation instance: {}", error.to_string())
            ReactiveFlowCreationError::ReactiveFlowConstructionError(error) => write!(f, "Failed to construct reactive flow: {}", error),
            ReactiveFlowCreationError::MissingVariable(variable_name) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type requires variable {} which wasn't provided",
                    variable_name
                )
            }
            ReactiveFlowCreationError::FlowTypeDoesntExist(flow_ty) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type {} doesn't exist",
                    flow_ty.type_definition().to_string()
                )
            }
            ReactiveFlowCreationError::EntityTypeDoesntExist(entity_ty) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type contains an entity instance of type {} which doesn't exist",
                    entity_ty.type_definition().to_string()
                )
            }
            ReactiveFlowCreationError::RelationTypeDoesntExist(relation_ty) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type contains a relation instance of type {} which doesn't exist",
                    relation_ty.type_definition().to_string()
                )
            }
            ReactiveFlowCreationError::InvalidOutboundId(id) => {
                write!(f, "Failed to construct reactive flow instance: Flow type provides relation instance which outbound refers to a entity instance with id {} which doesn't exist", id)
            }
            ReactiveFlowCreationError::InvalidInboundId(id) => {
                write!(f, "Failed to construct reactive flow instance: Flow type provides relation instance which inbound refers to entity instance with id {} which doesn't exist", id)
            }
        }
    }
}

impl From<ReactiveFlowCreationError> for String {
    fn from(e: ReactiveFlowCreationError) -> Self {
        format!("{}", e)
    }
}

#[derive(Debug)]
pub struct ReactiveFlowImportError;

#[async_trait]
pub trait ReactiveFlowManager: Send + Sync + Lifecycle {
    /// Returns true, if an flow instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the flow instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<ReactiveFlow>;

    /// Returns the flow instance that matches the given label or None.
    fn get_by_label(&self, label: &str) -> Option<ReactiveFlow>;

    /// Returns all reactive flow instances.
    fn get_all(&self) -> Vec<ReactiveFlow>;

    /// Returns the count of registered reactive flow instances.
    fn count_flow_instances(&self) -> usize;

    /// Creates a new reactive flow instance from the given flow instance descriptor.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// All reactive instances will be registered in the ReactiveEntityManager
    /// and the ReactiveRelationManager.
    fn create(&self, flow_instance: FlowInstance) -> Result<ReactiveFlow, ReactiveFlowCreationError>;

    /// Create a new reactive flow instance from the flow type by the given name.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// The properties are assigned to the wrapper entity instance.
    ///
    /// The variables will replace the property value.
    ///
    /// All reactive instances will be registered in the ReactiveEntityManager
    /// and the ReactiveRelationManager.
    fn create_from_type(
        &self,
        ty: &FlowTypeId,
        variables: HashMap<String, Value>,
        properties: HashMap<String, Value>,
    ) -> Result<ReactiveFlow, ReactiveFlowCreationError>;

    /// Registers the given reactive flow instance and registers all of the reactive instances
    /// contained in the given reactive flow instance.
    fn register_flow_instance_and_reactive_instances(&self, reactive_flow_instance: ReactiveFlow);

    /// Registers the given reactive flow instance. Does not register it's reactive instances except
    /// the wrapper entity.
    fn register_flow_instance(&self, reactive_flow_instance: ReactiveFlow);

    // /// The changes of the reactive flow instance with the given id will be written to graph database.
    // // TODO: return result
    // fn commit(&self, id: Uuid);

    /// Deletes the flow instance with the given id.
    fn delete(&self, id: Uuid) -> bool;

    // fn import(&self, path: &str) -> Result<ReactiveFlow, ReactiveFlowImportError>;
    //
    // // TODO: return result
    // fn export(&self, id: Uuid, path: &str);

    /// Registers a flow instance provider.
    fn add_provider(&self, id: Uuid, flow_instance_provider: Arc<dyn FlowInstanceProvider>);

    /// Unregisters a flow instance provider.
    fn remove_provider(&self, id: &Uuid);
}
