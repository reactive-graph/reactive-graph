use std::sync::Arc;

use async_graphql::*;

use reactive_graph_graph::ComponentOrEntityTypeId;
use reactive_graph_graph::Extension;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypeAddComponentError;
use reactive_graph_graph::RelationTypeAddExtensionError;
use reactive_graph_graph::RelationTypeAddPropertyError;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::RelationTypeRemoveComponentError;
use reactive_graph_graph::RelationTypeRemoveExtensionError;
use reactive_graph_graph::RelationTypeRemovePropertyError;
use reactive_graph_graph::RelationTypeUpdateError;
use reactive_graph_graph::RelationTypeUpdateExtensionError;
use reactive_graph_graph::RelationTypeUpdatePropertyError;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::mutation::ComponentOrEntityTypeIdDefinition;
use crate::mutation::ComponentTypeIdDefinition;
use crate::mutation::ComponentTypeIdDefinitions;
use crate::mutation::ExtensionTypeIdDefinition;
use crate::mutation::PropertyTypeDefinition;
use crate::mutation::PropertyTypeDefinitions;
use crate::mutation::RelationTypeIdDefinition;
use crate::query::GraphQLExtension;
use crate::query::GraphQLExtensions;
use crate::query::GraphQLRelationType;

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
        outbound_type: ComponentOrEntityTypeIdDefinition, // TODO: ComponentOrEntityTypeIdDefinition
        #[graphql(name = "type", desc = "The relation type.")] relation_type: RelationTypeIdDefinition,
        inbound_type: ComponentOrEntityTypeIdDefinition,
        #[graphql(desc = "Describes the relation type.")] description: Option<String>,
        #[graphql(desc = "Adds the given components to the newly created relation type.")] components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "The definitions of properties. These are added additionally to the properties provided by the given components.")] properties: Option<
            Vec<PropertyTypeDefinition>,
        >,
        #[graphql(desc = "The extension on the relation type.")] extensions: Option<Vec<GraphQLExtension>>,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;

        let outbound_type: ComponentOrEntityTypeId = outbound_type.into();
        let ty = relation_type.into();
        let inbound_type: ComponentOrEntityTypeId = inbound_type.into();

        let relation_type = RelationType::builder()
            .outbound_type(outbound_type)
            .ty(&ty)
            .inbound_type(inbound_type)
            .description(description.unwrap_or_default())
            .components(ComponentTypeIdDefinitions::new(components.unwrap_or_default()))
            .properties(PropertyTypeDefinitions::new(properties.unwrap_or_default()))
            .extensions(GraphQLExtensions::new(extensions.unwrap_or_default()))
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
        #[graphql(name = "type")] ty: RelationTypeIdDefinition,
        description: String,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        match relation_type_manager.update_description(&ty, &description) {
            Ok(_) => relation_type_manager
                .get(&ty)
                .map(|relation_type| relation_type.into())
                .ok_or_else(|| Error::new(format!("Relation Type {} not found", ty))),
            Err(RelationTypeUpdateError::RelationTypeDoesNotExist(ty)) => {
                Err(Error::new(format!("Failed to update description of relation type {ty}: Relation type does not exist")))
            }
        }
    }

    /// Adds the component with the given component_name to the relation type with the given name.
    async fn add_component(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        component: ComponentTypeIdDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = relation_type.into();
        let component_ty = component.into();
        if let Err(e) = relation_type_manager.add_component(&ty, &component_ty) {
            return Err(e.into());
        };
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeAddComponentError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    /// Remove the component with the given component_name from the relation type with the given name.
    async fn remove_component(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        component: ComponentTypeIdDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = relation_type.into();
        let component_ty = component.into();
        if let Err(e) = relation_type_manager.remove_component(&ty, &component_ty) {
            return Err(e.into());
        };
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeRemoveComponentError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    /// Adds a property to the relation type with the given name.
    async fn add_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = relation_type.into();
        if let Err(e) = relation_type_manager.add_property(&ty, property.into()) {
            return Err(e.into());
        }
        relation_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(RelationTypeAddPropertyError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    async fn update_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        property_name: String,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = relation_type.into();
        let property = property.into();
        if let Err(e) = relation_type_manager.update_property(&ty, property_name.as_str(), property) {
            return Err(e.into());
        }
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeUpdatePropertyError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the property with the given property_name from the relation type with the given name.
    async fn remove_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        property_name: String,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = relation_type.into();
        if let Err(e) = relation_type_manager.remove_property(&ty, property_name.as_str()) {
            return Err(e.into());
        }
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeRemovePropertyError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    /// Adds an extension to the relation type with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = relation_type.into();
        if let Err(e) = relation_type_manager.add_extension(&ty, extension.into()) {
            return Err(e.into());
        }
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeAddExtensionError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the extension with the given name of the flow type with the given name.
    async fn update_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: RelationTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty: RelationTypeId = ty.into();
        let extension: Extension = extension.into();
        if let Err(e) = relation_type_manager.update_extension(&ty, &extension.ty.clone(), extension) {
            return Err(e.into());
        }
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or(RelationTypeUpdateExtensionError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the extension with the given extension_name from the relation type with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: RelationTypeIdDefinition,
        #[graphql(name = "extension")] extension_ty: ExtensionTypeIdDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        if !relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {ty} does not exist")));
        }
        let extension_ty = extension_ty.into();
        if let Err(e) = relation_type_manager.remove_extension(&ty, &extension_ty) {
            return Err(e.into());
        }
        relation_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or(RelationTypeRemoveExtensionError::RelationTypeDoesNotExist(ty.clone()).into())
    }

    /// Deletes the relation type with the given name.
    async fn delete(&self, context: &Context<'_>, #[graphql(name = "type")] relation_type: RelationTypeIdDefinition) -> Result<bool> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        Ok(relation_type_manager.delete(&relation_type.into()).is_some())
    }
}
