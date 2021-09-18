use std::sync::Arc;

use async_graphql::*;
use log::debug;

use crate::api::EntityTypeManager;
use crate::builder::EntityTypeBuilder;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::query::{GraphQLEntityType, GraphQLExtension};

#[derive(Default)]
pub struct MutationEntityTypes;

/// Mutations for entity types
#[Object]
impl MutationEntityTypes {
    /// Creates a new entity type with the given name and components and properties.
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "The name of the entity type.")] name: String,
        #[graphql(desc = "The entity type belongs to this group.")] group: Option<String>,
        components: Option<Vec<String>>,
        behaviours: Option<Vec<String>>,
        #[graphql(
            desc = "The definitions of properties. These are added additionally to the properties provided by the given components."
        )]
        properties: Option<Vec<PropertyTypeDefinition>>,
        #[graphql(desc = "The extension on the entity type.")] extensions: Option<
            Vec<GraphQLExtension>,
        >,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;

        if entity_type_manager.has(name.clone()) {
            return Err(Error::new(format!(
                "Entity type {} already exists",
                name.clone()
            )));
        }

        let mut entity_type_builder = EntityTypeBuilder::new(name.clone());
        if group.is_some() {
            entity_type_builder.group(group.unwrap());
        }
        if components.is_some() {
            let components = components.unwrap();
            for component in components {
                entity_type_builder.component(component.clone());
            }
        }
        if behaviours.is_some() {
            let behaviours = behaviours.unwrap();
            for behaviour in behaviours {
                entity_type_builder.behaviour(behaviour.clone());
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

    // TODO: add component
    // TODO: remove component
    // TODO: add behaviour
    // TODO: remove behaviour
    // TODO: add property
    // TODO: remove property

    /// Deletes the entity type with the given name.
    async fn delete(&self, context: &Context<'_>, name: String) -> Result<bool> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        entity_type_manager.delete(name);
        Ok(true)
    }
}
