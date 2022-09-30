use std::sync::Arc;

use async_graphql::*;

use crate::api::ComponentExtensionError;
use crate::api::ComponentManager;
use crate::api::ComponentPropertyError;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::query::GraphQLComponent;
use crate::graphql::query::GraphQLExtension;

#[derive(Default)]
pub struct MutationComponents;

/// Mutations for components
#[Object]
impl MutationComponents {
    /// Creates a new component with the given name and properties.
    async fn create(
        &self,
        context: &Context<'_>,
        namespace: String,
        name: String,
        description: Option<String>,
        properties: Option<Vec<PropertyTypeDefinition>>,
        extensions: Option<Vec<GraphQLExtension>>,
    ) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let property_types = match properties {
            Some(properties) => properties.iter().map(|property| property.clone().into()).collect(),
            None => Vec::new(),
        };
        let extensions = match extensions {
            Some(extensions) => extensions.iter().map(|extension| extension.clone().into()).collect(),
            None => Vec::new(),
        };
        let component = crate::model::Component::new(namespace, name, description.unwrap_or_default(), property_types, extensions);
        component_manager.register(component.clone());
        Ok(component.into())
    }

    /// Adds a property to the component with the given name.
    async fn add_property(&self, context: &Context<'_>, name: String, property: PropertyTypeDefinition) -> Result<GraphQLComponent> {
        if let Ok(component_manager) = context.data::<Arc<dyn ComponentManager>>() {
            if !component_manager.has(name.as_str()) {
                return Err(Error::new(format!("Component {} does not exist", name)));
            }
            return match component_manager.add_property(name.as_str(), property.into()) {
                Ok(_) => component_manager
                    .get(name.as_str())
                    .map(|component| component.into())
                    .ok_or_else(|| Error::new(format!("Component {} not found", name))),
                Err(ComponentPropertyError::PropertyAlreadyExists) => {
                    Err(Error::new(format!("Failed to add property to component {}: Property already exists", name)))
                }
            };
        }
        Err(Error::new(format!("Failed to add property to component {}", name)))
    }

    /// Removes the property with the given property_name from the component with the given name.
    async fn remove_property(&self, context: &Context<'_>, name: String, property_name: String) -> Result<GraphQLComponent> {
        if let Ok(component_manager) = context.data::<Arc<dyn ComponentManager>>() {
            if !component_manager.has(name.as_str()) {
                return Err(Error::new(format!("Component {} does not exist", name)));
            }
            component_manager.remove_property(name.as_str(), property_name.as_str());
            return component_manager
                .get(name.as_str())
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {} not found", name)));
        }
        Err(Error::new(format!("Failed to remove property {} from component {}", property_name, name)))
    }

    /// Adds an extension to the component with the given name.
    async fn add_extension(&self, context: &Context<'_>, name: String, extension: GraphQLExtension) -> Result<GraphQLComponent> {
        if let Ok(component_manager) = context.data::<Arc<dyn ComponentManager>>() {
            if !component_manager.has(name.as_str()) {
                return Err(Error::new(format!("Component {} does not exist", name)));
            }
            return match component_manager.add_extension(name.as_str(), extension.into()) {
                Ok(_) => component_manager
                    .get(name.as_str())
                    .map(|component| component.into())
                    .ok_or_else(|| Error::new(format!("Component {} not found", name))),
                Err(ComponentExtensionError::ExtensionAlreadyExists) => {
                    Err(Error::new(format!("Failed to add extension to component {}: Extension already exists", name)))
                }
            };
        }
        Err(Error::new(format!("Failed to add extension to component {}", name)))
    }

    /// Removes the extension with the given extension_name from the component with the given name.
    async fn remove_extension(&self, context: &Context<'_>, name: String, extension_name: String) -> Result<GraphQLComponent> {
        if let Ok(component_manager) = context.data::<Arc<dyn ComponentManager>>() {
            if !component_manager.has(name.as_str()) {
                return Err(Error::new(format!("Component {} does not exist", name)));
            }
            component_manager.remove_extension(name.as_str(), extension_name.as_str());
            return component_manager
                .get(name.as_str())
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {} not found", name)));
        }
        Err(Error::new(format!("Failed to remove extension {} from component {}", extension_name, name)))
    }

    /// Deletes the component with the given name.
    async fn delete(&self, context: &Context<'_>, name: String) -> Result<bool> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        component_manager.delete(name.as_str());
        Ok(true)
    }
}
