use std::sync::Arc;

use async_graphql::*;

use crate::api::EntityBehaviourManager;
use crate::api::EntityTypeManager;
use crate::graphql::query::GraphQLBehaviour;
use crate::graphql::query::GraphQLEntityInstance;
use crate::graphql::query::GraphQLEntityType;
use crate::model::EntityBehaviourTypeId;
use crate::model::NamespacedTypeGetter;

pub struct GraphQLEntityBehaviour {
    entity_behaviour_ty: EntityBehaviourTypeId,
}

/// An entity behaviour.
#[Object(name = "EntityBehaviour")]
impl GraphQLEntityBehaviour {
    /// The entity type.
    async fn entity_type(&self, context: &Context<'_>) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let entity_type = entity_type_manager
            .get(&self.entity_behaviour_ty.entity_ty)
            .ok_or(Error::new(format!("Entity type {} does not exist!", &self.entity_behaviour_ty.entity_ty)))?;
        Ok(entity_type.into())
    }

    /// The namespace the behaviour type belongs to.
    async fn namespace(&self) -> String {
        self.entity_behaviour_ty.behaviour_ty.namespace()
    }

    /// The name of the behaviour type.
    async fn name(&self) -> String {
        self.entity_behaviour_ty.behaviour_ty.type_name()
    }

    /// The behaviour type.
    async fn behaviour(&self) -> GraphQLBehaviour {
        GraphQLBehaviour::from(self.entity_behaviour_ty.behaviour_ty.clone())
    }

    /// The instances with the behaviour.
    async fn instances(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityInstance>> {
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager>>()?;
        Ok(entity_behaviour_manager
            .get_instances_by_behaviour(&self.entity_behaviour_ty.behaviour_ty)
            .into_iter()
            .map(GraphQLEntityInstance::from)
            .collect())
    }
}

impl From<EntityBehaviourTypeId> for GraphQLEntityBehaviour {
    fn from(entity_behaviour_ty: EntityBehaviourTypeId) -> Self {
        GraphQLEntityBehaviour { entity_behaviour_ty }
    }
}
