use std::fmt;
use std::str::FromStr;

use async_graphql::*;
use indradb::EdgeKey;
use indradb::Type;
use uuid::Uuid;

/// The primary key of an edge consists of the outbound id, the
/// type name and the inbound id.
#[derive(Debug, Clone, InputObject)]
pub struct GraphQLEdgeKey {
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
        let t = Type::from_str(edge_key.type_name.as_str()).unwrap();
        EdgeKey {
            outbound_id: edge_key.outbound_id,
            t,
            inbound_id: edge_key.inbound_id,
        }
    }
}
