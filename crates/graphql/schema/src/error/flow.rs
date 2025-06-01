use std::fmt;

use uuid::Uuid;

use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::FlowTypeId;
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
    MissingFlowType(FlowTypeId),
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
            FlowMutationError::MissingFlow(id) => write!(f, "The flow {id} does not exist!"),
            FlowMutationError::FlowAlreadyExists(id) => {
                write!(f, "Can't create flow: The flow {id} already exist!")
            }
            FlowMutationError::EntityInstanceCreationError() => {
                write!(f, "Can't create entity instance")
            }
            FlowMutationError::RelationInstanceCreationError() => {
                write!(f, "Can't create relation instance")
            }
            // FlowMutationError::MissingWrapperEntityInstance(id) => write!(f, "Missing wrapper entity instance with the id {}", id),
            FlowMutationError::WrapperEntityInstanceAlreadyExists(id) => write!(f, "Can't create flow: An entity instance with the id {id} already exists!"),
            FlowMutationError::MissingEntityType(ty) => {
                write!(f, "Entity type {} does not exist", ty.type_definition())
            }
            FlowMutationError::MissingRelationType(ty) => {
                write!(f, "Relation type {} does not exist", ty.type_definition())
            }
            FlowMutationError::MissingFlowType(ty) => {
                write!(f, "Flow type {} does not exist", ty.type_definition())
            }
            FlowMutationError::MissingEntityInstance(id) => {
                write!(f, "Entity instance {id} does not exist")
            }
            FlowMutationError::MissingRelationInstance(relation_instance_id) => {
                write!(f, "Relation instance {relation_instance_id:?} does not exist")
            }
            FlowMutationError::MissingOutboundEntityInstance(id) => {
                write!(f, "Outbound entity instance {id} does not exist")
            }
            FlowMutationError::MissingInboundEntityInstance(id) => {
                write!(f, "Inbound entity instance {id} does not exist")
            }
            FlowMutationError::FlowInstanceDoesNotContainEntityInstance(id) => {
                write!(f, "Flow doesn't contain entity instance {id}")
            }
            FlowMutationError::FlowInstanceDoesNotContainRelationInstance(relation_instance_id) => {
                write!(f, "Flow doesn't contain relation instance {:?}", relation_instance_id.clone())
            }
        }
    }
}
