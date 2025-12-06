use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

use reactive_graph_graph::InstanceId;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_graph::RelationInstanceTypeIdError;
use reactive_graph_graph::RelationTypeId;

use crate::validator::InstanceIdValidator;
use crate::validator::NamespacedTypeValidator;

/// The primary key of a relation instance consists of the outbound id, the
/// type name, the inbound id and an instance_id.
#[derive(Serialize, Deserialize, Debug, Clone, InputObject)]
#[graphql(name = "RelationInstanceIdDefinition")]
pub struct GraphQLRelationInstanceId {
    /// The id of the outbound entity instance.
    pub outbound_id: Uuid,

    /// The fully qualified namespace of the relation type.
    #[graphql(name = "type", validator(custom = "NamespacedTypeValidator::new()"))]
    pub relation_type: String,

    /// The instance id.
    /// TODO: REGEX / InputValidator
    #[graphql(default, validator(custom = "InstanceIdValidator::new()"))]
    pub instance_id: String,

    /// The id of the inbound entity instance.
    pub inbound_id: Uuid,
}

impl GraphQLRelationInstanceId {
    pub fn ty(&self) -> Result<RelationInstanceTypeId, RelationInstanceTypeIdError> {
        Ok(RelationInstanceTypeId::new(
            RelationTypeId::from_str(&self.relation_type).map_err(RelationInstanceTypeIdError::NamespacedTypeParseError)?,
            InstanceId::from_str(&self.instance_id).map_err(RelationInstanceTypeIdError::InstanceIdError)?,
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
