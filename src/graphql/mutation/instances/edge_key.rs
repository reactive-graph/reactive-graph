use std::fmt;

use async_graphql::*;
use indradb::EdgeKey;
use uuid::Uuid;

use crate::model::fully_qualified_identifier;
use crate::model::NAMESPACE_RELATION_TYPE;

/// The primary key of an edge consists of the outbound id, the
/// type name and the inbound id.
#[derive(Debug, Clone, InputObject)]
#[graphql(name = "EdgeKeyDefinition")]
pub struct GraphQLEdgeKey {
    /// The namespace.
    pub namespace: String,

    /// The id of the outbound entity instance.
    pub outbound_id: Uuid,

    /// The name of the relation type.
    pub type_name: String,

    /// The id of the inbound entity instance.
    pub inbound_id: Uuid,
}

impl fmt::Display for GraphQLEdgeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<GraphQLEdgeKey> for EdgeKey {
    fn from(edge_key: GraphQLEdgeKey) -> Self {
        let t = fully_qualified_identifier(&edge_key.namespace, &edge_key.type_name, &NAMESPACE_RELATION_TYPE);
        EdgeKey {
            outbound_id: edge_key.outbound_id,
            t,
            inbound_id: edge_key.inbound_id,
        }
    }
}
