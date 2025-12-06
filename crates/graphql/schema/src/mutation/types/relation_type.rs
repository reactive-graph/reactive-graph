use std::str::FromStr;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypeAddComponentError;
use reactive_graph_graph::RelationTypeAddExtensionError;
use reactive_graph_graph::RelationTypeAddPropertyError;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::RelationTypeRemoveComponentError;
use reactive_graph_graph::RelationTypeRemoveExtensionError;
use reactive_graph_graph::RelationTypeRemovePropertyError;
use reactive_graph_graph::RelationTypeUpdateExtensionError;
use reactive_graph_graph::RelationTypeUpdatePropertyError;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::mutation::GraphQLExtensionDefinition;
use crate::mutation::GraphQLExtensionDefinitions;
use crate::mutation::GraphQLInboundOutboundType;
use crate::mutation::PropertyTypeDefinition;
use crate::mutation::PropertyTypeDefinitions;
use crate::query::GraphQLRelationType;
use crate::validator::NamespacedTypeValidator;

#[derive(Default)]
pub struct MutationRelationTypes;

/// Mutations for relation types
#[Object]
impl MutationRelationTypes {
    /// Creates a new relation type with the given name and components and properties.
    ///
    /// The outbound entity type and the inbound entity type must be specified.
    #[allow(clippy::too_many_arguments)]
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "The outbound type. Either an entity type or a component or any entity type or any component.")]
        outbound_type: GraphQLInboundOutboundType,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(desc = "The outbound type. Either an entity type or a component or any entity type or any component.")]
        inbound_type: GraphQLInboundOutboundType,
        #[graphql(desc = "Describes the relation type.")] description: Option<String>,
        #[graphql(desc = "Adds the given components to the newly created relation type.")] components: Option<Vec<String>>,
        #[graphql(desc = "The definitions of properties. These are added additionally to the properties provided by the given components.")] properties: Option<
            Vec<PropertyTypeDefinition>,
        >,
        #[graphql(desc = "The extensions of the relation type.")] extensions: Option<Vec<GraphQLExtensionDefinition>>,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let outbound_type = InboundOutboundType::try_from(outbound_type)?;
        let ty = RelationTypeId::from_str(&_type)?;
        let inbound_type = InboundOutboundType::try_from(inbound_type)?;
        let components = ComponentTypeIds::parse_optional_namespaces(components)?;
        let properties = PropertyTypeDefinitions::parse_optional_definitions(properties)?;
        let extensions = GraphQLExtensionDefinitions::parse_optional_definitions(extensions)?;
        let relation_type = RelationType::builder()
            .outbound_type(outbound_type)
            .ty(&ty)
            .inbound_type(inbound_type)
            .description(description.unwrap_or_default())
            .components(components)
            .properties(properties)
            .extensions(extensions)
            .build();
        match relation_type_manager.register(relation_type) {
            Ok(relation_type) => Ok(relation_type.into()),
            Err(e) => Err(e.into()),
        }
    }

    /// Updates the description of the given relation type.
    async fn update_description(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        description: String,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = RelationTypeId::from_str(&_type)?;
        Ok(relation_type_manager.update_description(&ty, &description)?.into())
    }

    /// Adds the component with the given component_name to the relation type with the given name.
    async fn add_component(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(name = "component")] component_namespace: String,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_ty = RelationTypeId::from_str(&_type)?;
        let component_ty = ComponentTypeId::from_str(&component_namespace)?;
        relation_type_manager.add_component(&relation_ty, &component_ty)?;
        relation_type_manager
            .get(&relation_ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeAddComponentError::RelationTypeDoesNotExist(relation_ty.clone()).into())
    }

    /// Remove the component with the given component_name from the relation type with the given name.
    async fn remove_component(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(name = "component", desc = "The fully qualified namespace of the component.")] component_namespace: String,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_ty = RelationTypeId::from_str(&_type)?;
        let component_ty = ComponentTypeId::from_str(&component_namespace)?;
        relation_type_manager.remove_component(&relation_ty, &component_ty)?;
        relation_type_manager
            .get(&relation_ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeRemoveComponentError::RelationTypeDoesNotExist(relation_ty.clone()).into())
    }

    /// Adds a property to the relation type with the given name.
    async fn add_property(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = RelationTypeId::from_str(&_type)?;
        let property = property.try_into()?;
        relation_type_manager.add_property(&ty, property)?;
        relation_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(RelationTypeAddPropertyError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    async fn update_property(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        property_name: String,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = RelationTypeId::from_str(&_type)?;
        let property = property.try_into()?;
        relation_type_manager.update_property(&ty, property_name.as_str(), property)?;
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeUpdatePropertyError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the property with the given property_name from the relation type with the given name.
    async fn remove_property(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        property_name: String,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = RelationTypeId::from_str(&_type)?;
        relation_type_manager.remove_property(&ty, property_name.as_str())?;
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeRemovePropertyError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    /// Adds an extension to the relation type with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        extension: GraphQLExtensionDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = RelationTypeId::from_str(&_type)?;
        let extension: Extension = extension.try_into()?;
        relation_type_manager.add_extension(&ty, extension)?;
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeAddExtensionError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the extension with the given name of the flow type with the given name.
    async fn update_extension(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(name = "extension", desc = "The fully qualified namespace of the extension.")] extension_namespace: String,
        extension: GraphQLExtensionDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_ty = RelationTypeId::from_str(&_type)?;
        let extension_ty = ExtensionTypeId::from_str(&extension_namespace)?;
        let extension = Extension::try_from(extension)?;
        relation_type_manager.update_extension(&relation_ty, &extension_ty, extension)?;
        relation_type_manager
            .get(&relation_ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeUpdateExtensionError::RelationTypeDoesNotExist(relation_ty.clone()).into())
    }

    /// Removes the extension with the given extension_name from the relation type with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(name = "extension", desc = "The fully qualified namespace of the extension.")] extension_namespace: String,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_ty = RelationTypeId::from_str(&_type)?;
        let extension_ty = ExtensionTypeId::from_str(&extension_namespace)?;
        relation_type_manager.remove_extension(&relation_ty, &extension_ty)?;
        relation_type_manager
            .get(&relation_ty)
            .map(|entity_type| entity_type.into())
            .ok_or(RelationTypeRemoveExtensionError::RelationTypeDoesNotExist(relation_ty.clone()).into())
    }

    /// Deletes the relation type with the given name.
    async fn delete(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
    ) -> Result<bool> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = RelationTypeId::from_str(&_type)?;
        Ok(relation_type_manager.delete(&ty.into()).is_some())
    }
}
