use std::sync::Arc;

use async_graphql::*;

use crate::api::ComponentManager;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::query::GraphQLComponent;

#[derive(Default)]
pub struct MutationComponents;

/// Mutations for components
#[Object]
impl MutationComponents {
    /// Creates a new component with the given name and properties.
    async fn create(&self, context: &Context<'_>, name: String, properties: Option<Vec<PropertyTypeDefinition>>) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let property_types = match properties {
            Some(properties) => properties.iter().map(|property| property.clone().into()).collect(),
            None => Vec::new(),
        };
        let component = crate::model::Component::new(name, property_types);
        component_manager.register(component.clone());
        Ok(component.into())
    }

    async fn delete(&self, context: &Context<'_>, name: String) -> Result<bool> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        component_manager.delete(name.as_str());
        Ok(true)
    }
}
