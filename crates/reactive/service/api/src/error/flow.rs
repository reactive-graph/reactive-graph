use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_graph::FlowTypeId;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_graph::TypeDefinitionGetter;
use inexor_rgf_reactive_model_api::ReactiveFlowConstructionError;
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
