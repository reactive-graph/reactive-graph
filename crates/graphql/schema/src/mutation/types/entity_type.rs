use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeAddComponentError;
use reactive_graph_graph::EntityTypeAddExtensionError;
use reactive_graph_graph::EntityTypeAddPropertyError;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::EntityTypeRemoveComponentError;
use reactive_graph_graph::EntityTypeRemoveExtensionError;
use reactive_graph_graph::EntityTypeRemovePropertyError;
use reactive_graph_graph::EntityTypeUpdateError;
use reactive_graph_graph::EntityTypeUpdateExtensionError;
use reactive_graph_graph::EntityTypeUpdatePropertyError;
use reactive_graph_graph::Extension;
use reactive_graph_type_system_api::EntityTypeManager;

use crate::mutation::ComponentTypeIdDefinition;
use crate::mutation::ComponentTypeIdDefinitions;
use crate::mutation::EntityTypeIdDefinition;
use crate::mutation::ExtensionTypeIdDefinition;
use crate::mutation::PropertyTypeDefinition;
use crate::mutation::PropertyTypeDefinitions;
use crate::query::GraphQLEntityType;
use crate::query::GraphQLExtension;
use crate::query::GraphQLExtensions;

#[derive(Default)]
pub struct MutationEntityTypes;

/// Mutations for entity types
#[Object]
impl MutationEntityTypes {
    /// Creates a new entity type with the given name and components and properties.
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        #[graphql(desc = "Describes the entity type.")] description: Option<String>,
        components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "The definitions of properties. These are added additionally to the properties provided by the given components.")] properties: Option<
            Vec<PropertyTypeDefinition>,
        >,
        #[graphql(desc = "The extension on the entity type.")] extensions: Option<Vec<GraphQLExtension>>,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        let entity_type = EntityType::builder()
            .ty(&ty)
            .description(description.unwrap_or_default())
            .components(ComponentTypeIdDefinitions::new(components.unwrap_or_default()))
            .properties(PropertyTypeDefinitions::new(properties.unwrap_or_default()))
            .extensions(GraphQLExtensions::new(extensions.unwrap_or_default()))
            .build();
        match entity_type_manager.register(entity_type) {
            Ok(entity_type) => Ok(entity_type.into()),
            Err(e) => Err(e.into()),
        }
    }

    /// Updates the description of the given entity type.
    async fn update_description(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        description: String,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        match entity_type_manager.update_description(&ty, &description) {
            Ok(_) => entity_type_manager
                .get(&ty)
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Entity Type {} not found", ty))),
            Err(EntityTypeUpdateError::EntityTypeDoesNotExist(ty)) => {
                Err(Error::new(format!("Failed to update description of entity type {ty}: Entity type does not exist")))
            }
        }
    }

    /// Adds the component with the given component_name to the entity type with the given name.
    async fn add_component(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        component: ComponentTypeIdDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        let component_ty = component.into();
        if let Err(e) = entity_type_manager.add_component(&ty, &component_ty) {
            return Err(e.into());
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeAddComponentError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Remove the component with the given component_name from the entity type with the given name.
    async fn remove_component(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        component: ComponentTypeIdDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        let component_ty = component.into();
        if let Err(e) = entity_type_manager.remove_component(&ty, &component_ty) {
            return Err(e.into());
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeRemoveComponentError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Adds a property to the entity type with the given name.
    async fn add_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        if let Err(e) = entity_type_manager.add_property(&ty, property.into()) {
            return Err(e.into());
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeAddPropertyError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the property with the given property_name from the entity type with the given name.
    async fn update_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        property_name: String,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        let property = property.into();
        if let Err(e) = entity_type_manager.update_property(&ty, property_name.as_str(), property) {
            return Err(e.into());
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeUpdatePropertyError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the property with the given property_name from the entity type with the given name.
    async fn remove_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        property_name: String,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        if let Err(e) = entity_type_manager.remove_property(&ty, property_name.as_str()) {
            return Err(e.into());
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeRemovePropertyError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Adds an extension to the entity type with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        // let extension: Extension = extension.into();
        if let Err(e) = entity_type_manager.add_extension(&ty, extension.into()) {
            return Err(e.into());
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeAddExtensionError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the extension with the given name of the flow type with the given name.
    async fn update_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty: EntityTypeId = ty.into();
        let extension: Extension = extension.into();
        if let Err(e) = entity_type_manager.update_extension(&ty, &extension.ty.clone(), extension) {
            return Err(e.into());
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeUpdateExtensionError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the extension with the given extension_name from the entity type with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        #[graphql(name = "extension")] extension_ty: ExtensionTypeIdDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        let extension_ty = extension_ty.into();
        if let Err(e) = entity_type_manager.remove_extension(&ty, &extension_ty) {
            return Err(e.into());
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeRemoveExtensionError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Deletes the entity type with the given name.
    async fn delete(&self, context: &Context<'_>, #[graphql(name = "type")] ty: EntityTypeIdDefinition) -> Result<bool> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        Ok(entity_type_manager.delete(&ty.into()).is_some())
    }
}
