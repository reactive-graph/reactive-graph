use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;

use crate::model::EntityTypeId;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "EntityTypeId")]
pub struct EntityTypeIdDefinition {
    /// The namespace of the entity type.
    pub namespace: String,

    /// The name of the entity type.
    #[graphql(name = "name")]
    pub type_name: String,
}

impl From<EntityTypeIdDefinition> for EntityTypeId {
    fn from(ty: EntityTypeIdDefinition) -> Self {
        EntityTypeId::new_from_type(ty.namespace, ty.type_name)
    }
}
