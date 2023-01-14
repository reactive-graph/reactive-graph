use std::sync::Arc;

use async_graphql::*;

use crate::api::ComponentExtensionError;
use crate::api::ComponentManager;
use crate::api::ComponentPropertyError;
use crate::api::ComponentRegistrationError;
use crate::graphql::mutation::ComponentTypeIdDefinition;
use crate::graphql::mutation::ExtensionTypeIdDefinition;
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
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
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
        let component = crate::model::Component::new(ty, description.unwrap_or_default(), property_types, extensions);
        match component_manager.register(component) {
            Ok(component) => Ok(component.into()),
            Err(ComponentRegistrationError::ComponentAlreadyExists(ty)) => {
                Err(Error::new(format!("Failed to create component {}: Component already exists", ty)))
            }
        }
    }

    /// Adds a property to the component with the given name.
    async fn add_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let ty = ty.into();
        if !component_manager.has(&ty) {
            return Err(Error::new(format!("Component {} does not exist", ty)));
        }
        return match component_manager.add_property(&ty, property.into()) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {} not found", ty))),
            Err(ComponentPropertyError::PropertyAlreadyExists) => {
                Err(Error::new(format!("Failed to add property to component {}: Property already exists", ty)))
            }
        };
    }

    /// Removes the property with the given property_name from the component with the given name.
    async fn remove_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
        property_name: String,
    ) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let ty = ty.into();
        if !component_manager.has(&ty) {
            return Err(Error::new(format!("Component {} does not exist", ty)));
        }
        component_manager.remove_property(&ty, property_name.as_str());
        return component_manager
            .get(&ty)
            .map(|component| component.into())
            .ok_or_else(|| Error::new(format!("Component {} not found", ty)));
    }

    /// Adds an extension to the component with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLComponent> {
        let ty = ty.into();
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        if !component_manager.has(&ty) {
            return Err(Error::new(format!("Component {} does not exist", ty)));
        }
        return match component_manager.add_extension(&ty, extension.into()) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {} not found", ty))),
            Err(ComponentExtensionError::ExtensionAlreadyExists) => {
                Err(Error::new(format!("Failed to add extension to component {}: Extension already exists", ty)))
            }
        };
    }

    /// Removes the extension with the given extension_name from the component with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
        #[graphql(name = "extension")] extension_ty: ExtensionTypeIdDefinition,
    ) -> Result<GraphQLComponent> {
        let ty = ty.into();
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        if !component_manager.has(&ty) {
            return Err(Error::new(format!("Component {} does not exist", ty)));
        }
        let extension_ty = extension_ty.into();
        component_manager.remove_extension(&ty, &extension_ty);
        return component_manager
            .get(&ty)
            .map(|component| component.into())
            .ok_or_else(|| Error::new(format!("Component {} not found", ty)));
    }

    /// Deletes the component with the given name.
    async fn delete(&self, context: &Context<'_>, #[graphql(name = "type")] ty: ComponentTypeIdDefinition) -> Result<bool> {
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        component_manager.delete(&ty.into());
        Ok(true)
    }
}
