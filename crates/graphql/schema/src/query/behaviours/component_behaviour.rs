use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;

use inexor_rgf_behaviour_model_api::ComponentBehaviourTypeId;
use inexor_rgf_graph::NamespacedTypeGetter;
use inexor_rgf_type_system_api::ComponentManager;

use crate::query::GraphQLBehaviour;
use crate::query::GraphQLComponent;

pub struct GraphQLComponentBehaviour {
    component_behaviour_ty: ComponentBehaviourTypeId,
}

/// A component behaviour.
#[Object(name = "ComponentBehaviour")]
impl GraphQLComponentBehaviour {
    /// The component.
    async fn component(&self, context: &Context<'_>) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
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
