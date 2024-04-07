use std::fmt;

use uuid::Uuid;

use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::TypeDefinitionGetter;

#[derive(Debug)]
pub enum FlowMutationError {
    MissingFlow(Uuid),
    FlowAlreadyExists(Uuid),
    EntityInstanceCreationError(),
    RelationInstanceCreationError(),
    // MissingWrapperEntityInstance(Uuid),
    WrapperEntityInstanceAlreadyExists(Uuid),
    MissingEntityType(EntityTypeId),
    MissingRelationType(RelationTypeId),
    MissingEntityInstance(Uuid),
    MissingRelationInstance(RelationInstanceId),
    MissingOutboundEntityInstance(Uuid),
    MissingInboundEntityInstance(Uuid),
    FlowInstanceDoesNotContainEntityInstance(Uuid),
    FlowInstanceDoesNotContainRelationInstance(RelationInstanceId),
}

impl fmt::Display for FlowMutationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FlowMutationError::MissingFlow(id) => write!(f, "The flow {} does not exist!", id),
            FlowMutationError::FlowAlreadyExists(id) => {
                write!(f, "Can't create flow: The flow {} already exist!", id)
            }
            FlowMutationError::EntityInstanceCreationError() => {
                write!(f, "Can't create entity instance")
            }
            FlowMutationError::RelationInstanceCreationError() => {
                write!(f, "Can't create relation instance")
            }
            // FlowMutationError::MissingWrapperEntityInstance(id) => write!(f, "Missing wrapper entity instance with the id {}", id),
            FlowMutationError::WrapperEntityInstanceAlreadyExists(id) => write!(f, "Can't create flow: An entity instance with the id {} already exists!", id),
            FlowMutationError::MissingEntityType(ty) => {
                write!(f, "Entity type {} does not exist", ty.type_definition().to_string())
            }
            FlowMutationError::MissingRelationType(ty) => {
                write!(f, "Relation type {} does not exist", ty.type_definition().to_string())
            }
            FlowMutationError::MissingEntityInstance(id) => {
                write!(f, "Entity instance {} does not exist", id)
            }
            FlowMutationError::MissingRelationInstance(edge_key) => {
                write!(f, "Relation instance {:?} does not exist", edge_key)
            }
            FlowMutationError::MissingOutboundEntityInstance(id) => {
                write!(f, "Outbound entity instance {} does not exist", id)
            }
            FlowMutationError::MissingInboundEntityInstance(id) => {
                write!(f, "Inbound entity instance {} does not exist", id)
            }
            FlowMutationError::FlowInstanceDoesNotContainEntityInstance(id) => {
                write!(f, "Flow doesn't contain entity instance {}", id)
            }
            FlowMutationError::FlowInstanceDoesNotContainRelationInstance(edge_key) => {
                write!(f, "Flow doesn't contain relation instance {:?}", edge_key.clone())
            }
        }
    }
}
