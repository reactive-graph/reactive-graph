use std::sync::Arc;

use async_graphql::*;

use crate::api::RelationBehaviourManager;
use crate::api::RelationTypeManager;
use crate::graphql::query::GraphQLBehaviour;
use crate::graphql::query::GraphQLRelationInstance;
use crate::graphql::query::GraphQLRelationType;
use crate::model::NamespacedTypeGetter;
use crate::reactive::RelationBehaviourTypeId;

pub struct GraphQLRelationBehaviour {
    relation_behaviour_ty: RelationBehaviourTypeId,
}

/// A relation behaviour.
#[Object(name = "RelationBehaviour")]
impl GraphQLRelationBehaviour {
    /// The relation type.
    async fn relation_type(&self, context: &Context<'_>) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let relation_type = relation_type_manager
            .get(&self.relation_behaviour_ty.relation_ty)
            .ok_or(Error::new(format!("Relation type {} does not exist!", &self.relation_behaviour_ty.relation_ty)))?;
        Ok(relation_type.into())
    }

    /// The namespace the behaviour type belongs to.
    async fn namespace(&self) -> String {
        self.relation_behaviour_ty.behaviour_ty.namespace()
    }

    /// The name of the behaviour type.
    async fn name(&self) -> String {
        self.relation_behaviour_ty.behaviour_ty.type_name()
    }

    /// The behaviour type.
    async fn behaviour(&self) -> GraphQLBehaviour {
        GraphQLBehaviour::from(self.relation_behaviour_ty.behaviour_ty.clone())
    }

    /// The instances with the behaviour.
    async fn instances(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationInstance>> {
        let relation_behaviour_manager = context.data::<Arc<dyn RelationBehaviourManager>>()?;
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
