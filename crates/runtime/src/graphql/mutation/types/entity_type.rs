use std::sync::Arc;

use async_graphql::*;

use crate::api::EntityTypeManager;
use crate::error::types::entity::EntityTypeRegistrationError;
use crate::graphql::mutation::ComponentTypeIdDefinition;
use crate::graphql::mutation::ComponentTypeIdDefinitions;
use crate::graphql::mutation::PropertyTypeDefinitions;
use crate::graphql::mutation::EntityTypeIdDefinition;
use crate::graphql::mutation::ExtensionTypeIdDefinition;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::query::GraphQLExtensions;
use crate::graphql::query::GraphQLEntityType;
use crate::graphql::query::GraphQLExtension;
use crate::model::AddExtensionError;
use crate::model::AddPropertyError;
use crate::model::EntityType;
use crate::model::EntityTypeAddComponentError;
use crate::model::EntityTypeAddExtensionError;
use crate::model::EntityTypeAddPropertyError;
use crate::model::Extension;

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
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        if entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {} already exists", &ty)));
        }
        let entity_type = EntityType::builder()
            .ty(&ty)
            .description(description.unwrap_or_default())
            .components(ComponentTypeIdDefinitions::new(components.unwrap_or_default()))
            .properties(PropertyTypeDefinitions::new(properties.unwrap_or_default()))
            .extensions(GraphQLExtensions::new(extensions.unwrap_or_default()))
            .build();
        match entity_type_manager.register(entity_type) {
            Ok(entity_type) => Ok(entity_type.into()),
            Err(EntityTypeRegistrationError::EntityTypeAlreadyExists(ty)) => {
                Err(Error::new(format!("Failed to create entity type {}: Entity type already exists", ty)))
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
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        let component_ty = component.into();
        // if !entity_type_manager.has(&ty) {
        //     return Err(Error::new(format!("Entity type {ty} does not exist")));
        // }
        match entity_type_manager.add_component(&ty, &component_ty) {
            Ok(_) => entity_type_manager
                .get(&ty)
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Entity type {ty} not found"))),
            Err(EntityTypeAddComponentError::IsAlreadyA(component_ty)) => Err(Error::new(format!("Entity type {ty} already has component {component_ty}"))),
            Err(EntityTypeAddComponentError::EntityTypeDoesNotExist(entity_ty)) => Err(Error::new(format!("Entity type {entity_ty} doesn't exist"))),
            Err(EntityTypeAddComponentError::ComponentDoesNotExist(component_ty)) => Err(Error::new(format!("Component {component_ty} doesn't exist"))),
        }
    }

    /// Remove the component with the given component_name from the entity type with the given name.
    async fn remove_component(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        component: ComponentTypeIdDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        let component_ty = component.into();
        if !entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {} does not exist", ty)));
        }
        if let Err(e) = entity_type_manager.remove_component(&ty, &component_ty) {
            return Err(Error::new(format!("Failed to remove component {component_ty} from entity type {ty}")));
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or_else(|| Error::new(format!("Entity type {} not found", ty)))
    }

    /// Adds a property to the entity type with the given name.
    async fn add_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        // if !entity_type_manager.has(&ty) {
        //     return Err(Error::new(format!("Entity type {} does not exist", ty)));
        // }
        match entity_type_manager.add_property(&ty, property.into()) {
            Ok(_) => entity_type_manager
                .get(&ty)
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Entity type {ty} not found"))),
            Err(EntityTypeAddPropertyError::EntityTypeDoesNotExist(entity_ty)) => {
                Err(Error::new(format!("Entity type {entity_ty} does not exist")))
            }
            Err(EntityTypeAddPropertyError::AddPropertyError(AddPropertyError::PropertyAlreadyExist(property_name))) => {
                Err(Error::new(format!("Failed to add property to entity type {ty}: Property {property_name} already exists")))
            }
        }
    }

    /// Removes the property with the given property_name from the entity type with the given name.
    async fn remove_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        property_name: String,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        if !entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {ty} does not exist")));
        }
        if entity_type_manager.remove_property(&ty, property_name.as_str()).is_err() {
            return Err(Error::new(format!("Failed to remove property {property_name} from entity type {ty}")));
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or_else(|| Error::new(format!("Entity type {} not found", ty)))
    }

    /// Adds an extension to the entity type with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        // if !entity_type_manager.has(&ty) {
        //     return Err(Error::new(format!("Entity type {} does not exist", ty)));
        // }
        let extension: Extension = extension.into();
        match entity_type_manager.add_extension(&ty, extension) {
            Ok(_) => entity_type_manager
                .get(&ty)
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Entity type {} not found", ty))),
            Err(EntityTypeAddExtensionError::EntityTypeDoesNotExist(entity_ty)) => Err(Error::new(format!("Entity type {entity_ty} does not exist"))),
            Err(EntityTypeAddExtensionError::AddExtensionError(AddExtensionError::ExtensionAlreadyExist(extension_ty))) => Err(Error::new(format!(
                "Failed to add extension {extension_ty} to entity type {ty}: Extension already exists"
            ))),
        }
    }

    // TODO: async fn update_extension()

    /// Removes the extension with the given extension_name from the entity type with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: EntityTypeIdDefinition,
        #[graphql(name = "extension")] extension_ty: ExtensionTypeIdDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let ty = ty.into();
        if !entity_type_manager.has(&ty) {
            return Err(Error::new(format!("Entity type {ty} does not exist")));
        }
        let extension_ty = extension_ty.into();
        if entity_type_manager.remove_extension(&ty, &extension_ty).is_err() {
            return Err(Error::new(format!("Failed to remove extension {extension_ty} from entity type {ty}")));
        }
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or_else(|| Error::new(format!("Entity type {ty} not found")))
    }

    /// Deletes the entity type with the given name.
    async fn delete(&self, context: &Context<'_>, #[graphql(name = "type")] ty: EntityTypeIdDefinition) -> Result<bool> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        Ok(entity_type_manager.delete(&ty.into()).is_some())
    }
}
