use async_graphql::InputObject;
use uuid::Uuid;

use reactive_graph_graph::InstanceId;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_graph::RelationInstanceTypeIdError;
use reactive_graph_graph::RelationTypeId;

/// The primary key of a relation instance consists of the outbound id, the
/// type name, the inbound id and an instance_id.
#[derive(Debug, Clone, InputObject)]
#[graphql(name = "RelationInstanceIdDefinition")]
pub struct GraphQLRelationInstanceId {
    /// The id of the outbound entity instance.
    pub outbound_id: Uuid,

    /// The fully qualified namespace of the relation type.
    /// TODO: REGEX
    #[graphql(name = "name")]
    pub namespace: String,

    /// The instance id.
    /// TODO: REGEX
    #[graphql(default)]
    pub instance_id: String,

    /// The id of the inbound entity instance.
    pub inbound_id: Uuid,
}

impl GraphQLRelationInstanceId {
    pub fn ty(&self) -> Result<RelationInstanceTypeId, RelationInstanceTypeIdError> {
        Ok(RelationInstanceTypeId::new(
            RelationTypeId::parse_namespace(&self.namespace).map_err(RelationInstanceTypeIdError::NamespacedTypeError)?,
            InstanceId::try_from(&self.instance_id).map_err(RelationInstanceTypeIdError::InstanceIdError)?,
        ))
    }
}

impl TryFrom<GraphQLRelationInstanceId> for RelationInstanceId {
    type Error = RelationInstanceTypeIdError;

    fn try_from(relation_instance_id: GraphQLRelationInstanceId) -> Result<Self, Self::Error> {
        Ok(RelationInstanceId::new(
            relation_instance_id.outbound_id,
            relation_instance_id.ty()?,
            relation_instance_id.inbound_id,
        ))
    }
}

impl TryFrom<&GraphQLRelationInstanceId> for RelationInstanceId {
    type Error = RelationInstanceTypeIdError;

    fn try_from(relation_instance_id: &GraphQLRelationInstanceId) -> Result<Self, Self::Error> {
        Ok(RelationInstanceId::new(
            relation_instance_id.outbound_id,
            relation_instance_id.ty()?,
            relation_instance_id.inbound_id,
        ))
    }
}
