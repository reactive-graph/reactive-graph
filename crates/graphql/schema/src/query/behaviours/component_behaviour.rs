use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_behaviour_model_api::ComponentBehaviourTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_type_system_api::ComponentManager;

use crate::query::GraphQLBehaviour;
use crate::query::GraphQLComponent;
use crate::query::GraphQLNamespacedType;

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

    // TODO: this is already available via the behaviour resolver below
    /// The namespace and type name.
    #[graphql(name = "type")]
    async fn ty(&self) -> GraphQLNamespacedType {
        self.component_behaviour_ty.behaviour_ty.namespaced_type().into()
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
