use std::fmt;

use async_graphql::*;
use uuid::Uuid;
use crate::model::RelationInstanceId;
use inexor_rgf_core_model::TYPE_ID_TYPE_SEPARATOR;

use crate::model::RelationInstanceTypeId;
// use crate::model::TypeDefinitionGetter;

/// The primary key of an edge consists of the outbound id, the
/// type name and the inbound id.
#[derive(Debug, Clone, InputObject)]
#[graphql(name = "EdgeKeyDefinition")]
pub struct GraphQLEdgeKey {
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

impl GraphQLEdgeKey {
    pub fn ty(&self) -> RelationInstanceTypeId {
        RelationInstanceTypeId::new_from_type_unique_for_instance_id(&self.namespace, &self.type_name, &self.instance_id)
    }
}

impl fmt::Display for GraphQLEdgeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}--{}{}{}__{}--{}", self.outbound_id, self.namespace, &TYPE_ID_TYPE_SEPARATOR, self.type_name, self.instance_id, self.outbound_id)
    }
}

impl From<GraphQLEdgeKey> for RelationInstanceId {
    fn from(edge_key: GraphQLEdgeKey) -> Self {
        RelationInstanceId::new(edge_key.outbound_id, edge_key.ty().clone(), edge_key.inbound_id)
    }
}

impl From<&GraphQLEdgeKey> for RelationInstanceId {
    fn from(edge_key: &GraphQLEdgeKey) -> Self {
        RelationInstanceId::new(edge_key.outbound_id, edge_key.ty().clone(), edge_key.inbound_id)
    }
}
