use std::fmt;

use async_graphql::*;
use uuid::Uuid;

use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_graph::TYPE_ID_TYPE_SEPARATOR;

/// The primary key of a relation instance consists of the outbound id, the
/// type name, the inbound id and an instance_id.
#[derive(Debug, Clone, InputObject)]
#[graphql(name = "RelationInstanceIdDefinition")]
pub struct GraphQLRelationInstanceId {
    /// The id of the outbound entity instance.
    pub outbound_id: Uuid,

    /// The namespace.
    pub namespace: String,

    /// The name of the relation type.
    pub type_name: String,

    /// The instance id.
    #[graphql(default)]
    pub instance_id: String,

    /// The id of the inbound entity instance.
    pub inbound_id: Uuid,
}

impl GraphQLRelationInstanceId {
    pub fn ty(&self) -> RelationInstanceTypeId {
        RelationInstanceTypeId::new_from_type_unique_for_instance_id(&self.namespace, &self.type_name, &self.instance_id)
    }
}

impl fmt::Display for GraphQLRelationInstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}--{}{}{}{}{}--{}",
            self.outbound_id, self.namespace, &TYPE_ID_TYPE_SEPARATOR, self.type_name, TYPE_ID_TYPE_SEPARATOR, self.instance_id, self.outbound_id
        )
    }
}

impl From<GraphQLRelationInstanceId> for RelationInstanceId {
    fn from(relation_instance_id: GraphQLRelationInstanceId) -> Self {
        RelationInstanceId::new(relation_instance_id.outbound_id, relation_instance_id.ty().clone(), relation_instance_id.inbound_id)
    }
}

impl From<&GraphQLRelationInstanceId> for RelationInstanceId {
    fn from(relation_instance_id: &GraphQLRelationInstanceId) -> Self {
        RelationInstanceId::new(relation_instance_id.outbound_id, relation_instance_id.ty().clone(), relation_instance_id.inbound_id)
    }
}
