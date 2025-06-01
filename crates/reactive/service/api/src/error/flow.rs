use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_reactive_model_api::ReactiveFlowConstructionError;
use std::fmt;
use uuid::Uuid;

// TODO: thiserror
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
                write!(f, "The UUID {id} has been already taken!")
            }
            ReactiveFlowCreationError::MissingWrapperInstance => {
                write!(f, "The created wrapper instance cannot be found")
            }
            // ReactiveFlowCreationError::ReactiveEntityCreationError(error) => write!(f, "Failed to create reactive entity instance: {}", error.to_string()),
            // ReactiveFlowCreationError::ReactiveRelationCreationError(error) => write!(f, "Failed to create reactive relation instance: {}", error.to_string())
            ReactiveFlowCreationError::ReactiveFlowConstructionError(e) => write!(f, "Failed to construct reactive flow: {e}"),
            ReactiveFlowCreationError::MissingVariable(variable_name) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type requires variable {variable_name} which wasn't provided"
                )
            }
            ReactiveFlowCreationError::FlowTypeDoesntExist(flow_ty) => {
                write!(f, "Failed to construct reactive flow instance: Flow type {} doesn't exist", flow_ty.type_definition())
            }
            ReactiveFlowCreationError::EntityTypeDoesntExist(entity_ty) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type contains an entity instance of type {} which doesn't exist",
                    entity_ty.type_definition()
                )
            }
            ReactiveFlowCreationError::RelationTypeDoesntExist(relation_ty) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type contains a relation instance of type {} which doesn't exist",
                    relation_ty.type_definition()
                )
            }
            ReactiveFlowCreationError::InvalidOutboundId(id) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type provides relation instance which outbound refers to a entity instance with id {id} which doesn't exist"
                )
            }
            ReactiveFlowCreationError::InvalidInboundId(id) => {
                write!(
                    f,
                    "Failed to construct reactive flow instance: Flow type provides relation instance which inbound refers to entity instance with id {id} which doesn't exist",
                )
            }
        }
    }
}

impl From<ReactiveFlowCreationError> for String {
    fn from(e: ReactiveFlowCreationError) -> Self {
        format!("{e}")
    }
}

#[derive(Debug)]
pub struct ReactiveFlowImportError;
