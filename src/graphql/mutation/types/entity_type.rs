use std::sync::Arc;

use async_graphql::*;
use log::debug;

use crate::api::EntityTypeComponentError;
use crate::api::EntityTypeExtensionError;
use crate::api::EntityTypeManager;
use crate::api::EntityTypePropertyError;
use crate::api::EntityTypeRegistrationError;
use crate::builder::EntityTypeBuilder;
use crate::graphql::mutation::ComponentTypeIdDefinition;
use crate::graphql::mutation::EntityTypeIdDefinition;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::query::GraphQLEntityType;
use crate::graphql::query::GraphQLExtension;

#[derive(Default)]
pub struct MutationEntityTypes;

/// Mutations for entity types
#[Object]
impl MutationEntityTypes {
    /// Creates a new entity type with the given name and components and properties.
    async fn create(
        &self,
        context: &Context<'_>,
        ty: EntityTypeIdDefinition,
        components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "The definitions of properties. These are added additionally to the properties provided by the given components.")] properties: Option<
            Vec<PropertyTypeDefinition>,
        >,
        #[graphql(desc = "The extension on the entity type.")] extensions: Option<Vec<GraphQLExtension>>,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        if entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {} already exists", &ty)));
        }
        let mut entity_type_builder = EntityTypeBuilder::new(&ty);
        if let Some(components) = components {
            for component in components {
                entity_type_builder.component(component);
            }
        }
        if let Some(properties) = properties {
            for property in properties {
                debug!("{} {}", property.name, property.data_type.to_string());
                entity_type_builder.property_from(property.clone());
            }
        }
        if let Some(extensions) = extensions {
            for extension in extensions {
                debug!("{} {}", extension.name, extension.extension.to_string());
                entity_type_builder.extension(extension.name, extension.extension.clone());
            }
        }

        let entity_type = entity_type_builder.build();
        match entity_type_manager.register(entity_type) {
            Ok(entity_type) => Ok(entity_type.into()),
            Err(EntityTypeRegistrationError::EntityTypeAlreadyExists(ty)) => {
                Err(Error::new(format!("Failed to create entity type {}: Entity type already exists", ty)))
            }
        }
    }

    /// Adds the component with the given component_name to the entity type with the given name.
    async fn add_component(&self, context: &Context<'_>, ty: EntityTypeIdDefinition, component: ComponentTypeIdDefinition) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        let component_ty = component.into();
        if !entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {} does not exist", ty)));
        }
        return match entity_type_manager.add_component(&ty, &component_ty) {
            Ok(_) => entity_type_manager
                .get(&ty)
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Entity type {} not found", &ty))),
            Err(EntityTypeComponentError::ComponentAlreadyAssigned) => Err(Error::new(format!("Entity type {} already has component {}", &ty, &component_ty))),
            Err(EntityTypeComponentError::ComponentDoesNotExist) => Err(Error::new(format!("Component {} doesn't exist", component_ty))),
        };
    }

    /// Remove the component with the given component_name from the entity type with the given name.
    async fn remove_component(&self, context: &Context<'_>, ty: EntityTypeIdDefinition, component: ComponentTypeIdDefinition) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        let component_ty = component.into();
        if !entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {} does not exist", ty)));
        }
        entity_type_manager.remove_component(&ty, &component_ty);
        return entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or_else(|| Error::new(format!("Entity type {} not found", ty)));
    }

    /// Adds a property to the entity type with the given name.
    async fn add_property(&self, context: &Context<'_>, ty: EntityTypeIdDefinition, property: PropertyTypeDefinition) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        if !entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {} does not exist", ty)));
        }
        return match entity_type_manager.add_property(&ty, property.into()) {
            Ok(_) => entity_type_manager
                .get(&ty)
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Entity type {} not found", ty))),
            Err(EntityTypePropertyError::PropertyAlreadyExists) => {
                Err(Error::new(format!("Failed to add property to entity type {}: Property already exists", ty)))
            }
        };
    }

    /// Removes the property with the given property_name from the entity type with the given name.
    async fn remove_property(&self, context: &Context<'_>, ty: EntityTypeIdDefinition, property_name: String) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        if !entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {} does not exist", ty)));
        }
        entity_type_manager.remove_property(&ty, property_name.as_str());
        return entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or_else(|| Error::new(format!("Entity type {} not found", ty)));
    }

    /// Adds an extension to the entity type with the given name.
    async fn add_extension(&self, context: &Context<'_>, ty: EntityTypeIdDefinition, extension: GraphQLExtension) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        if !entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {} does not exist", ty)));
        }
        return match entity_type_manager.add_extension(&ty, extension.into()) {
            Ok(_) => entity_type_manager
                .get(&ty)
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Entity type {} not found", ty))),
            Err(EntityTypeExtensionError::ExtensionAlreadyExists) => {
                Err(Error::new(format!("Failed to add extension to component {}: Extension already exists", ty)))
            }
        };
    }

    /// Removes the extension with the given extension_name from the entity type with the given name.
    async fn remove_extension(&self, context: &Context<'_>, ty: EntityTypeIdDefinition, extension_name: String) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        if !entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {} does not exist", ty)));
        }
        entity_type_manager.remove_extension(&ty, extension_name.as_str());
        return entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or_else(|| Error::new(format!("Entity type {} not found", ty)));
    }

    /// Deletes the entity type with the given name.
    async fn delete(&self, context: &Context<'_>, ty: EntityTypeIdDefinition) -> Result<bool> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        entity_type_manager.delete(&ty.into());
        Ok(true)
    }
}
