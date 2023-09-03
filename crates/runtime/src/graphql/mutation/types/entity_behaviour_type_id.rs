use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;

use crate::graphql::mutation::BehaviourTypeIdDefinition;
use crate::graphql::mutation::EntityTypeIdDefinition;
use crate::reactive::BehaviourTypeId;
use crate::reactive::EntityBehaviourTypeId;
use crate::model::EntityTypeId;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "EntityBehaviourTypeId")]
pub struct EntityBehaviourTypeIdDefinition {
    /// The entity type.
    pub entity: EntityTypeIdDefinition,

    /// The behaviour.
    pub behaviour: BehaviourTypeIdDefinition,
}

impl From<EntityBehaviourTypeIdDefinition> for EntityBehaviourTypeId {
    fn from(ty: EntityBehaviourTypeIdDefinition) -> Self {
        let entity_ty = EntityTypeId::from(ty.entity);
        let behaviour_ty = BehaviourTypeId::from(ty.behaviour);
        EntityBehaviourTypeId::new(entity_ty, behaviour_ty)
    }
}

impl From<EntityBehaviourTypeId> for EntityBehaviourTypeIdDefinition {
    fn from(ty: EntityBehaviourTypeId) -> Self {
        let entity_ty = EntityTypeIdDefinition::from(ty.entity_ty);
        let behaviour_ty = BehaviourTypeIdDefinition::from(ty.behaviour_ty);
        EntityBehaviourTypeIdDefinition {
            entity: entity_ty,
            behaviour: behaviour_ty,
        }
    }
}
