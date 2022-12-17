use std::sync::Arc;

use async_graphql::*;

use crate::api::ComponentManager;
use crate::graphql::query::GraphQLBehaviour;
use crate::graphql::query::GraphQLComponent;
use crate::model::ComponentBehaviourTypeId;
use crate::model::NamespacedTypeGetter;

pub struct GraphQLComponentBehaviour {
    component_behaviour_ty: ComponentBehaviourTypeId,
}

/// A component behaviour.
#[Object(name = "ComponentBehaviour")]
impl GraphQLComponentBehaviour {
    /// The component.
    async fn component(&self, context: &Context<'_>) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let component = component_manager
            .get(&self.component_behaviour_ty.component_ty)
            .ok_or(Error::new(format!("Component {} does not exist!", &self.component_behaviour_ty.component_ty)))?;
        Ok(component.into())
    }

    /// The namespace the behaviour type belongs to.
    async fn namespace(&self) -> String {
        self.component_behaviour_ty.behaviour_ty.namespace()
    }

    /// The name of the behaviour type.
    async fn name(&self) -> String {
        self.component_behaviour_ty.behaviour_ty.type_name()
    }

    /// The behaviour type.
    async fn behaviour(&self) -> GraphQLBehaviour {
        GraphQLBehaviour::from(self.component_behaviour_ty.behaviour_ty.clone())
    }
}

impl From<ComponentBehaviourTypeId> for GraphQLComponentBehaviour {
    fn from(component_behaviour_ty: ComponentBehaviourTypeId) -> Self {
        GraphQLComponentBehaviour { component_behaviour_ty }
    }
}
