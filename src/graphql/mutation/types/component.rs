use std::sync::Arc;

use async_graphql::*;

use crate::api::ComponentManager;
use crate::graphql::mutation::PropertyTypeDefinition;

#[derive(Default)]
pub struct MutationComponents;

/// Mutations for components
#[Object]
impl MutationComponents {
    /// Creates a new component with the given name and properties.
    async fn create(
        &self,
        context: &Context<'_>,
        name: String,
        properties: Option<Vec<PropertyTypeDefinition>>,
    ) -> Result<crate::model::Component> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let property_types;
        if properties.is_some() {
            property_types = properties
                .unwrap()
                .iter()
                .map(|property| property.clone().into())
                .collect();
        } else {
            property_types = Vec::new();
        }
        let component = crate::model::Component::new(name, property_types);
        component_manager.register(component.clone());
        Ok(component)
    }

    // TODO: add property
    // TODO: remove property

    async fn delete(&self, context: &Context<'_>, name: String) -> Result<bool> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        component_manager.delete(name);
        Ok(true)
    }
}
