use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;

use reactive_graph_graph::RelationTypeId;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "RelationTypeId")]
pub struct RelationTypeIdDefinition {
    /// The namespace of the relation type.
    pub namespace: String,

    /// The name of the relation type.
    #[graphql(name = "name")]
    pub type_name: String,
}

impl From<RelationTypeIdDefinition> for RelationTypeId {
    fn from(ty: RelationTypeIdDefinition) -> Self {
        RelationTypeId::new_from_type(ty.namespace, ty.type_name)
    }
}
