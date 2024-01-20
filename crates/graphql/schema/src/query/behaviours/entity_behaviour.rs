use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;

use inexor_rgf_behaviour_model_api::EntityBehaviourTypeId;
use inexor_rgf_behaviour_service_api::EntityBehaviourManager;
use inexor_rgf_graph::NamespacedTypeGetter;
use inexor_rgf_type_system_api::EntityTypeManager;

use crate::query::GraphQLBehaviour;
use crate::query::GraphQLEntityInstance;
use crate::query::GraphQLEntityType;

pub struct GraphQLEntityBehaviour {
    entity_behaviour_ty: EntityBehaviourTypeId,
}

/// An entity behaviour.
#[Object(name = "EntityBehaviour")]
impl GraphQLEntityBehaviour {
    /// The entity type.
    async fn entity_type(&self, context: &Context<'_>) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
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
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager + Send + Sync>>()?;
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
