use std::sync::Arc;

use async_graphql::*;
use inexor_rgf_rt_api::ComponentRegistrationError;

use crate::api::ComponentManager;
use crate::graphql::mutation::ComponentTypeIdDefinition;
use crate::graphql::mutation::ExtensionTypeIdDefinition;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::query::GraphQLComponent;
use crate::graphql::query::GraphQLExtension;
use inexor_rgf_graph::AddExtensionError;
use inexor_rgf_graph::AddPropertyError;
use inexor_rgf_graph::ComponentAddExtensionError;
use inexor_rgf_graph::ComponentAddPropertyError;
use inexor_rgf_graph::ComponentRemoveExtensionError;
use inexor_rgf_graph::ComponentRemovePropertyError;
use inexor_rgf_graph::ComponentUpdateExtensionError;
use inexor_rgf_graph::ComponentUpdatePropertyError;
use inexor_rgf_graph::RemoveExtensionError;
use inexor_rgf_graph::RemovePropertyError;
use inexor_rgf_graph::UpdateExtensionError;
use inexor_rgf_graph::UpdatePropertyError;

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
        let component = inexor_rgf_graph::Component::new(ty, description.unwrap_or_default(), property_types, extensions);
        match component_manager.register(component) {
            Ok(component) => Ok(component.into()),
            Err(ComponentRegistrationError::ComponentAlreadyExists(ty)) => {
                Err(Error::new(format!("Failed to create component {}: Component already exists", ty)))
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
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let ty = ty.into();
        let Some(mut component) = component_manager.get(&ty) else {
            return Err(Error::new(format!("Failed to rename component {ty}: Component does not exist")));
        };
        component.ty = new_ty.into();
        component_manager.replace(&ty, component.clone());
        Ok(component.into())
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
        match component_manager.add_property(&ty, property.into()) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {} not found", ty))),
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
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let ty = ty.into();
        match component_manager.update_property(&ty, &property_name, property.into()) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {} not found", ty))),
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
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let ty = ty.into();
        match component_manager.remove_property(&ty, property_name.as_str()) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {} not found", ty))),
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
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        match component_manager.add_extension(&ty, extension.into()) {
            Ok(_) => component_manager
                .get(&ty)
                .map(|component| component.into())
                .ok_or_else(|| Error::new(format!("Component {} not found", ty))),
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
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
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
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
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
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        Ok(component_manager.delete(&ty.into()))
    }
}
