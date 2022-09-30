use std::sync::Arc;

use async_graphql::*;
use log::debug;

use crate::api::EntityTypeComponentError;
use crate::api::EntityTypeExtensionError;
use crate::api::EntityTypeManager;
use crate::api::EntityTypePropertyError;
use crate::builder::EntityTypeBuilder;
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
        #[graphql(desc = "The namespace the entity type belongs to.")] namespace: Option<String>,
        #[graphql(desc = "The name of the entity type.")] name: String,
        components: Option<Vec<String>>,
        #[graphql(desc = "The definitions of properties. These are added additionally to the properties provided by the given components.")] properties: Option<
            Vec<PropertyTypeDefinition>,
        >,
        #[graphql(desc = "The extension on the entity type.")] extensions: Option<Vec<GraphQLExtension>>,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;

        if entity_type_manager.has(&name) {
            return Err(Error::new(format!("Entity type {} already exists", name)));
        }

        let namespace = namespace.unwrap_or_default();
        let mut entity_type_builder = EntityTypeBuilder::new(namespace, name);
        if components.is_some() {
            let components = components.unwrap();
            for component in components {
                entity_type_builder.component(component.clone());
            }
        }
        if properties.is_some() {
            for property in properties.unwrap() {
                debug!("{} {}", property.name, property.data_type.to_string());
                entity_type_builder.property_from(property.clone());
            }
        }
        if extensions.is_some() {
            for extension in extensions.unwrap() {
                debug!("{} {}", extension.name, extension.extension.to_string());
                entity_type_builder.extension(extension.name, extension.extension.clone());
            }
        }

        let entity_type = entity_type_builder.build();
        entity_type_manager.register(entity_type.clone());
        Ok(entity_type.into())
    }

    /// Adds the component with the given component_name to the entity type with the given name.
    async fn add_component(&self, context: &Context<'_>, name: String, component_name: String) -> Result<GraphQLEntityType> {
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager>>() {
            if !entity_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Entity type {} does not exist", name)));
            }
            return match entity_type_manager.add_component(name.as_str(), component_name.as_str()) {
                Ok(_) => entity_type_manager
                    .get(name.as_str())
                    .map(|entity_type| entity_type.into())
                    .ok_or_else(|| Error::new(format!("Entity type {} not found", name))),
                Err(EntityTypeComponentError::ComponentAlreadyAssigned) => {
                    Err(Error::new(format!("Entity type {} has already component {}", name, component_name)))
                }
                Err(EntityTypeComponentError::ComponentDoesNotExist) => Err(Error::new(format!("Component {} doesn't exist", component_name))),
            };
        }
        Err(Error::new(format!("Failed to add component {} to entity type {}", component_name, name)))
    }

    /// Remove the component with the given component_name from the entity type with the given name.
    async fn remove_component(&self, context: &Context<'_>, name: String, component_name: String) -> Result<GraphQLEntityType> {
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager>>() {
            if !entity_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Entity type {} does not exist", name)));
            }
            entity_type_manager.remove_component(name.as_str(), component_name.as_str());
            return entity_type_manager
                .get(name.as_str())
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Entity type {} not found", name)));
        }
        Err(Error::new(format!("Failed to add component {} to entity type {}", component_name, name)))
    }

    /// Adds a property to the entity type with the given name.
    async fn add_property(&self, context: &Context<'_>, name: String, property: PropertyTypeDefinition) -> Result<GraphQLEntityType> {
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager>>() {
            if !entity_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Entity type {} does not exist", name)));
            }
            return match entity_type_manager.add_property(name.as_str(), property.into()) {
                Ok(_) => entity_type_manager
                    .get(name.as_str())
                    .map(|entity_type| entity_type.into())
                    .ok_or_else(|| Error::new(format!("Entity type {} not found", name))),
                Err(EntityTypePropertyError::PropertyAlreadyExists) => {
                    Err(Error::new(format!("Failed to add property to entity type {}: Property already exists", name)))
                }
            };
        }
        Err(Error::new(format!("Failed to add property to entity type {}", name)))
    }

    /// Removes the property with the given property_name from the entity type with the given name.
    async fn remove_property(&self, context: &Context<'_>, name: String, property_name: String) -> Result<GraphQLEntityType> {
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager>>() {
            if !entity_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Entity type {} does not exist", name)));
            }
            entity_type_manager.remove_property(name.as_str(), property_name.as_str());
            return entity_type_manager
                .get(name.as_str())
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Entity type {} not found", name)));
        }
        Err(Error::new(format!("Failed to remove property {} from entity type {}", property_name, name)))
    }

    /// Adds an extension to the entity type with the given name.
    async fn add_extension(&self, context: &Context<'_>, name: String, extension: GraphQLExtension) -> Result<GraphQLEntityType> {
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager>>() {
            if !entity_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Entity type {} does not exist", name)));
            }
            return match entity_type_manager.add_extension(name.as_str(), extension.into()) {
                Ok(_) => entity_type_manager
                    .get(name.as_str())
                    .map(|entity_type| entity_type.into())
                    .ok_or_else(|| Error::new(format!("Entity type {} not found", name))),
                Err(EntityTypeExtensionError::ExtensionAlreadyExists) => {
                    Err(Error::new(format!("Failed to add extension to component {}: Extension already exists", name)))
                }
            };
        }
        Err(Error::new(format!("Failed to add extension to entity type {}", name)))
    }

    /// Removes the extension with the given extension_name from the entity type with the given name.
    async fn remove_extension(&self, context: &Context<'_>, name: String, extension_name: String) -> Result<GraphQLEntityType> {
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager>>() {
            if !entity_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Entity type {} does not exist", name)));
            }
            entity_type_manager.remove_extension(name.as_str(), extension_name.as_str());
            return entity_type_manager
                .get(name.as_str())
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Entity type {} not found", name)));
        }
        Err(Error::new(format!("Failed to remove extension {} from entity type {}", extension_name, name)))
    }

    /// Deletes the entity type with the given name.
    async fn delete(&self, context: &Context<'_>, name: String) -> Result<bool> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        entity_type_manager.delete(name.as_str());
        Ok(true)
    }
}
