use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;

use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;

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

impl From<EntityTypeId> for EntityTypeIdDefinition {
    fn from(ty: EntityTypeId) -> Self {
        EntityTypeIdDefinition {
            namespace: ty.namespace(),
            type_name: ty.type_name(),
        }
    }
}
