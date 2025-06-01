use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_graph::AddExtensionError;
use reactive_graph_graph::AddPropertyError;
use reactive_graph_graph::ComponentAddExtensionError;
use reactive_graph_graph::ComponentAddPropertyError;
use reactive_graph_graph::ComponentRemoveExtensionError;
use reactive_graph_graph::ComponentRemovePropertyError;
use reactive_graph_graph::ComponentUpdateError;
use reactive_graph_graph::ComponentUpdateExtensionError;
use reactive_graph_graph::ComponentUpdatePropertyError;
use reactive_graph_graph::RemoveExtensionError;
use reactive_graph_graph::RemovePropertyError;
use reactive_graph_graph::UpdateExtensionError;
use reactive_graph_graph::UpdatePropertyError;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::ComponentRegistrationError;

use crate::mutation::ComponentTypeIdDefinition;
use crate::mutation::ExtensionTypeIdDefinition;
use crate::mutation::PropertyTypeDefinition;
use crate::query::GraphQLComponent;
use crate::query::GraphQLExtension;

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
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let property_types = match properties {
            Some(properties) => properties.iter().map(|property| property.clone().into()).collect(),
            None => Vec::new(),
        };
        let extensions = match extensions {
            Some(extensions) => extensions.iter().map(|extension| extension.clone().into()).collect(),
            None => Vec::new(),
        };
        let component = reactive_graph_graph::Component::new(ty, description.unwrap_or_default(), property_types, extensions);
        match component_manager.register(component) {
            Ok(component) => Ok(component.into()),
            Err(ComponentRegistrationError::ComponentAlreadyExists(ty)) => {
                Err(Error::new(format!("Failed to create component {ty}: Component already exists")))
            }
        }
    }

    /// Renames the component with the given type to the component with the given new type.
    async fn rename(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
        #[graphql(name = "newType")] new_ty: ComponentTypeIdDefinition,
    ) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let ty = ty.into();
        let Some(mut component) = component_manager.get(&ty) else {
            return Err(Error::new(format!("Failed to rename component {ty}: Component does not exist")));
        };
        component.ty = new_ty.into();
        component_manager.replace(&ty, component.clone());
        Ok(component.into())
    }

    /// Updates the description of the given component.
    async fn update_description(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
        description: String,
    ) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let ty = ty.into();
        match component_manager.update_description(&ty, &description) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {ty} not found"))),
            Err(ComponentUpdateError::ComponentDoesNotExist(ty)) => {
                Err(Error::new(format!("Failed to update description of component {ty}: Component does not exist")))
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
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let ty = ty.into();
        match component_manager.add_property(&ty, property.into()) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {ty} not found"))),
            Err(ComponentAddPropertyError::ComponentDoesNotExist(ty)) => {
                Err(Error::new(format!("Failed to add property to component {ty}: Component does not exist")))
            }
            Err(ComponentAddPropertyError::AddPropertyError(AddPropertyError::PropertyAlreadyExist(property_name))) => {
                Err(Error::new(format!("Failed to add property to component {ty}: Property {property_name} already exists")))
            }
        }
    }

    /// Updates the property with the given name of the given component.
    async fn update_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
        #[graphql(name = "name")] property_name: String,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let ty = ty.into();
        match component_manager.update_property(&ty, &property_name, property.into()) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {ty} not found"))),
            Err(ComponentUpdatePropertyError::ComponentDoesNotExist(ty)) => {
                Err(Error::new(format!("Failed to update property of component {ty}: Component does not exist")))
            }
            Err(ComponentUpdatePropertyError::UpdatePropertyError(UpdatePropertyError::PropertyDoesNotExist(property_name))) => {
                Err(Error::new(format!("Failed to update property of component {ty}: Property {property_name} does not exist")))
            }
        }
    }

    /// Removes the property with the given property_name from the component with the given name.
    async fn remove_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
        property_name: String,
    ) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let ty = ty.into();
        match component_manager.remove_property(&ty, property_name.as_str()) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {ty} not found"))),
            Err(ComponentRemovePropertyError::ComponentDoesNotExist(ty)) => {
                Err(Error::new(format!("Failed to update property of component {ty}: Component does not exist")))
            }
            Err(ComponentRemovePropertyError::RemovePropertyError(RemovePropertyError::PropertyDoesNotExist(property_name))) => {
                Err(Error::new(format!("Failed to remove property of component {ty}: Property {property_name} does not exist")))
            }
        }
    }

    /// Adds an extension to the component with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLComponent> {
        let ty = ty.into();
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        match component_manager.add_extension(&ty, extension.into()) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {ty} not found"))),
            Err(ComponentAddExtensionError::ComponentDoesNotExist(ty)) => {
                Err(Error::new(format!("Failed to add extension to component {ty}: Component does not exist")))
            }
            Err(ComponentAddExtensionError::AddExtensionError(AddExtensionError::ExtensionAlreadyExist(extension_ty))) => {
                Err(Error::new(format!("Failed to add extension to component {ty}: Extension {extension_ty} already exists")))
            }
        }
    }

    /// Updates the extension with the given id of the given component.
    async fn update_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] component_ty: ComponentTypeIdDefinition,
        #[graphql(name = "extension_type")] extension_ty: ExtensionTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLComponent> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let component_ty = component_ty.into();
        let extension_ty = extension_ty.into();
        match component_manager.update_extension(&component_ty, &extension_ty, extension.into()) {
            Ok(_) => component_manager
                .get(&component_ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {component_ty} not found"))),
            Err(ComponentUpdateExtensionError::ComponentDoesNotExist(component_ty)) => {
                Err(Error::new(format!("Failed to update extension of component {component_ty}: Component does not exist")))
            }
            Err(ComponentUpdateExtensionError::UpdateExtensionError(UpdateExtensionError::ExtensionDoesNotExist(extension_ty))) => Err(Error::new(format!(
                "Failed to update extension of component {component_ty}: Extension {extension_ty} does not exist"
            ))),
        }
    }

    /// Removes the extension with the given extension_name from the component with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: ComponentTypeIdDefinition,
        #[graphql(name = "extension")] extension_ty: ExtensionTypeIdDefinition,
    ) -> Result<GraphQLComponent> {
        let ty = ty.into();
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let extension_ty = extension_ty.into();
        match component_manager.remove_extension(&ty, &extension_ty) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {ty} not found"))),
            Err(ComponentRemoveExtensionError::ComponentDoesNotExist(ty)) => {
                Err(Error::new(format!("Failed to remove extension of component {ty}: Component does not exist")))
            }
            Err(ComponentRemoveExtensionError::RemoveExtensionError(RemoveExtensionError::ExtensionDoesNotExist(extension_ty))) => {
                Err(Error::new(format!("Failed to remove extension of component {ty}: Extension {extension_ty} does not exist")))
            }
        }
    }

    /// Deletes the component with the given name.
    async fn delete(&self, context: &Context<'_>, #[graphql(name = "type")] ty: ComponentTypeIdDefinition) -> Result<bool> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        Ok(component_manager.delete(&ty.into()))
    }
}
