use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_behaviour_model_api::RelationBehaviourTypeId;
use reactive_graph_behaviour_service_api::RelationBehaviourManager;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::query::GraphQLBehaviour;
use crate::query::GraphQLRelationInstance;
use crate::query::GraphQLRelationType;

pub struct GraphQLRelationBehaviour {
    relation_behaviour_ty: RelationBehaviourTypeId,
}

/// A relation behaviour.
#[Object(name = "RelationBehaviour")]
impl GraphQLRelationBehaviour {
    /// The relation type.
    async fn relation_type(&self, context: &Context<'_>) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_type = relation_type_manager
            .get(&self.relation_behaviour_ty.relation_ty)
            .ok_or(Error::new(format!("Relation type {} does not exist!", &self.relation_behaviour_ty.relation_ty)))?;
        Ok(relation_type.into())
    }

    /// The behaviour type.
    async fn behaviour(&self) -> GraphQLBehaviour {
        GraphQLBehaviour::from(self.relation_behaviour_ty.behaviour_ty.clone())
    }

    /// The instances with the behaviour.
    async fn instances(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationInstance>> {
        let relation_behaviour_manager = context.data::<Arc<dyn RelationBehaviourManager + Send + Sync>>()?;
        Ok(relation_behaviour_manager
            .get_instances_by_behaviour(&self.relation_behaviour_ty.behaviour_ty)
            .into_iter()
            .map(GraphQLRelationInstance::from)
            .collect())
    }
}

impl From<RelationBehaviourTypeId> for GraphQLRelationBehaviour {
    fn from(relation_behaviour_ty: RelationBehaviourTypeId) -> Self {
        GraphQLRelationBehaviour { relation_behaviour_ty }
    }
}
