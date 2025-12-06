use std::str::FromStr;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeAddComponentError;
use reactive_graph_graph::EntityTypeAddExtensionError;
use reactive_graph_graph::EntityTypeAddPropertyError;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::EntityTypeRemoveComponentError;
use reactive_graph_graph::EntityTypeRemoveExtensionError;
use reactive_graph_graph::EntityTypeRemovePropertyError;
use reactive_graph_graph::EntityTypeUpdateExtensionError;
use reactive_graph_graph::EntityTypeUpdatePropertyError;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_type_system_api::EntityTypeManager;

use crate::mutation::GraphQLExtensionDefinition;
use crate::mutation::GraphQLExtensionDefinitions;
use crate::mutation::PropertyTypeDefinition;
use crate::mutation::PropertyTypeDefinitions;
use crate::query::GraphQLEntityType;
use crate::validator::NamespacedTypeValidator;

#[derive(Default)]
pub struct MutationEntityTypes;

/// Mutations for entity types
#[Object]
impl MutationEntityTypes {
    /// Creates a new entity type with the given name and components and properties.
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(desc = "Textual description of the entity type.")] description: Option<String>,
        #[graphql(desc = "Adds the given components to the newly created entity type.")] components: Option<Vec<String>>,
        #[graphql(desc = "The definitions of properties. These are added additionally to the properties provided by the given components.")] properties: Option<
            Vec<PropertyTypeDefinition>,
        >,
        #[graphql(desc = "The extensions of the entity type.")] extensions: Option<Vec<GraphQLExtensionDefinition>>,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = EntityTypeId::from_str(&_type)?;
        let components = ComponentTypeIds::parse_optional_namespaces(components)?;
        let properties = PropertyTypeDefinitions::parse_optional_definitions(properties)?;
        let extensions = GraphQLExtensionDefinitions::parse_optional_definitions(extensions)?;

        let entity_type = EntityType::builder()
            .ty(&ty)
            .description(description.unwrap_or_default())
            .components(components)
            .properties(properties)
            .extensions(extensions)
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
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        description: String,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = EntityTypeId::from_str(&_type)?;
        Ok(entity_type_manager.update_description(&ty, &description)?.into())
    }

    /// Adds the component with the given component_name to the entity type with the given name.
    async fn add_component(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(name = "component", desc = "The fully qualified namespace of the component.")] component_namespace: String,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let entity_ty = EntityTypeId::from_str(&_type)?;
        let component_ty = ComponentTypeId::from_str(&component_namespace)?;
        entity_type_manager.add_component(&entity_ty, &component_ty)?;
        entity_type_manager
            .get(&entity_ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeAddComponentError::EntityTypeDoesNotExist(entity_ty.clone()).into())
    }

    /// Remove the component with the given component_name from the entity type with the given name.
    async fn remove_component(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(name = "component", desc = "The fully qualified namespace of the component.")] component_namespace: String,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let entity_ty = EntityTypeId::from_str(&_type)?;
        let component_ty = ComponentTypeId::from_str(&component_namespace)?;
        entity_type_manager.remove_component(&entity_ty, &component_ty)?;
        entity_type_manager
            .get(&entity_ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeRemoveComponentError::EntityTypeDoesNotExist(entity_ty.clone()).into())
    }

    /// Adds a property to the entity type with the given name.
    async fn add_property(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = EntityTypeId::from_str(&_type)?;
        let property = property.try_into()?;
        entity_type_manager.add_property(&ty, property)?;
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeAddPropertyError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the property with the given property_name from the entity type with the given name.
    async fn update_property(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        property_name: String,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = EntityTypeId::from_str(&_type)?;
        let property = property.try_into()?;
        entity_type_manager.update_property(&ty, property_name.as_str(), property)?;
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeUpdatePropertyError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the property with the given property_name from the entity type with the given name.
    async fn remove_property(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        property_name: String,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = EntityTypeId::from_str(&_type)?;
        entity_type_manager.remove_property(&ty, property_name.as_str())?;
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeRemovePropertyError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Adds an extension to the entity type with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        extension: GraphQLExtensionDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = EntityTypeId::from_str(&_type)?;
        let extension = Extension::try_from(extension)?;
        entity_type_manager.add_extension(&ty, extension)?;
        entity_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeAddExtensionError::EntityTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the extension with the given name of the entity type with the given name.
    async fn update_extension(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(name = "extension", desc = "The fully qualified namespace of the extension.")] extension_namespace: String,
        extension: GraphQLExtensionDefinition,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let entity_ty = EntityTypeId::from_str(&_type)?;
        let extension_ty = ExtensionTypeId::from_str(&extension_namespace)?;
        let extension = Extension::try_from(extension)?;
        entity_type_manager.update_extension(&entity_ty, &extension_ty, extension)?;
        entity_type_manager
            .get(&entity_ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeUpdateExtensionError::EntityTypeDoesNotExist(entity_ty.clone()).into())
    }

    /// Removes the extension with the given extension_name from the entity type with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(name = "extension", desc = "The fully qualified namespace of the extension.")] extension_namespace: String,
    ) -> Result<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let entity_ty = EntityTypeId::from_str(&_type)?;
        let extension_ty = ExtensionTypeId::from_str(&extension_namespace)?;
        entity_type_manager.remove_extension(&entity_ty, &extension_ty)?;
        entity_type_manager
            .get(&entity_ty)
            .map(|entity_type| entity_type.into())
            .ok_or(EntityTypeRemoveExtensionError::EntityTypeDoesNotExist(entity_ty.clone()).into())
    }

    /// Deletes the entity type with the given name.
    async fn delete(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
    ) -> Result<bool> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let ty = EntityTypeId::from_str(&_type)?;
        Ok(entity_type_manager.delete(&ty.into()).is_some())
    }
}
