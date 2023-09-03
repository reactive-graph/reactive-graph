use std::sync::Arc;

use async_graphql::*;

use crate::api::EntityTypeManager;
use crate::api::RelationTypeManager;
use crate::error::types::relation::RelationTypeRegistrationError;
use crate::graphql::mutation::{ComponentTypeIdDefinition, ComponentTypeIdDefinitions, PropertyTypeDefinitions};
use crate::graphql::mutation::EntityTypeIdDefinition;
use crate::graphql::mutation::ExtensionTypeIdDefinition;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::mutation::RelationTypeIdDefinition;
use crate::graphql::query::{GraphQLExtension, GraphQLExtensions};
use crate::graphql::query::GraphQLRelationType;
use crate::model::AddExtensionError;
use crate::model::AddPropertyError;
use crate::model::RelationType;
use crate::model::RelationTypeAddComponentError;
use crate::model::RelationTypeAddExtensionError;
use crate::model::RelationTypeAddPropertyError;

#[derive(Default)]
pub struct MutationRelationTypes;

/// Mutations for relation types
#[Object]
impl MutationRelationTypes {
    /// Creates a new relation type with the given name and components and properties.
    ///
    /// The outbound entity type and the inbound entity type must be specified.
    async fn create(
        &self,
        context: &Context<'_>,
        outbound_type: EntityTypeIdDefinition, // TODO: ComponentOrEntityTypeIdDefinition
        #[graphql(name = "type", desc = "The relation type.")] relation_type: RelationTypeIdDefinition,
        inbound_type: EntityTypeIdDefinition, // TODO: ComponentOrEntityTypeIdDefinition
        #[graphql(desc = "Describes the relation type.")] description: Option<String>,
        #[graphql(desc = "Adds the given components to the newly created relation type.")] components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "The definitions of properties. These are added additionally to the properties provided by the given components.")] properties: Option<
            Vec<PropertyTypeDefinition>,
        >,
        #[graphql(desc = "The extension on the relation type.")] extensions: Option<Vec<GraphQLExtension>>,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;

        let outbound_ty = outbound_type.into();
        let ty = relation_type.into();
        let inbound_ty = inbound_type.into();

        if relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {} already exists", ty)));
        }
        if !entity_type_manager.has(&outbound_ty) {
            return Err(Error::new(format!("Outbound entity type {} does not exist", outbound_ty)));
        }
        if !entity_type_manager.has(&inbound_ty) {
            return Err(Error::new(format!("Inbound entity type {} does not exist", inbound_ty)));
        }

        let relation_type = RelationType::builder()
            .outbound_type(outbound_ty)
            .ty(&ty)
            .inbound_type(inbound_ty)
            .description(description.unwrap_or_default())
            .components(ComponentTypeIdDefinitions::new(components.unwrap_or_default()))
            .properties(PropertyTypeDefinitions::new(properties.unwrap_or_default()))
            .extensions(GraphQLExtensions::new(extensions.unwrap_or_default()))
            .build();
        // new(outbound_ty, &ty, inbound_ty);
        // if let Some(description) = description {
        //     relation_type_builder.description(description);
        // }
        // if components.is_some() {
        //     for component in components.unwrap() {
        //         debug!("Add component {}", &component);
        //         relation_type_builder.component(component);
        //     }
        // }
        // if properties.is_some() {
        //     for property in properties.unwrap() {
        //         debug!("Add property {} {} {}", property.name, property.data_type.to_string(), property.socket_type.to_string());
        //         relation_type_builder.property_from(property.clone());
        //     }
        // }
        // if extensions.is_some() {
        //     for extension in extensions.unwrap() {
        //         debug!("Add extension {} {}", &extension.ty, extension.extension.to_string());
        //         relation_type_builder.extension(extension.ty.namespace, extension.ty.type_name, extension.extension.clone());
        //     }
        // }
        //
        // let relation_type = relation_type_builder.build();
        match relation_type_manager.register(relation_type) {
            Ok(relation_type) => Ok(relation_type.into()),
            Err(RelationTypeRegistrationError::RelationTypeAlreadyExists(ty)) => {
                Err(Error::new(format!("Failed to create relation type {}: relation type already exists", ty)))
            }
            Err(RelationTypeRegistrationError::OutboundComponentDoesNotExist(ty, component_ty)) => Err(Error::new(format!(
                "Failed to create relation type {}: outbound component {} does not exist",
                ty, component_ty
            ))),
            Err(RelationTypeRegistrationError::OutboundEntityTypeDoesNotExist(ty, outbound_ty)) => Err(Error::new(format!(
                "Failed to create relation type {}: outbound entity type {} does not exist",
                ty, outbound_ty
            ))),
            Err(RelationTypeRegistrationError::InboundComponentDoesNotExist(ty, component_ty)) => Err(Error::new(format!(
                "Failed to create relation type {}: inbound component {} does not exist",
                ty, component_ty
            ))),
            Err(RelationTypeRegistrationError::InboundEntityTypeDoesNotExist(ty, inbound_ty)) => Err(Error::new(format!(
                "Failed to create relation type {}: inbound entity type {} does not exist",
                ty, inbound_ty
            ))),
        }
    }

    /// Adds the component with the given component_name to the relation type with the given name.
    async fn add_component(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        component: ComponentTypeIdDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        let component_ty = component.into();
        match relation_type_manager.add_component(&ty, &component_ty) {
            Ok(_) => relation_type_manager
                .get(&ty)
                .map(|relation_type| relation_type.into())
                .ok_or_else(|| Error::new(format!("Relation type {} not found", ty))),
            Err(RelationTypeAddComponentError::RelationTypeDoesNotExist(relation_ty)) => {
                Err(Error::new(format!("Relation type {relation_ty} does not exist")))
            }
            Err(RelationTypeAddComponentError::IsAlreadyA(component_ty)) => {
                Err(Error::new(format!("Relation type {ty} has already component {component_ty}")))
            }
            Err(RelationTypeAddComponentError::ComponentDoesNotExist(component_ty)) => Err(Error::new(format!("Component {component_ty} doesn't exist"))),
        }
    }

    /// Remove the component with the given component_name from the relation type with the given name.
    async fn remove_component(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        component: ComponentTypeIdDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        let component_ty = component.into();
        if !relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {ty} does not exist")));
        }
        if relation_type_manager.remove_component(&ty, &component_ty).is_err() {
            return Err(Error::new(format!("Failed to remove component {component_ty} from relation type {ty}")));
        };
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or_else(|| Error::new(format!("Relation type {ty} not found")))
    }

    /// Adds a property to the relation type with the given name.
    async fn add_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        match relation_type_manager.add_property(&ty, property.into()) {
            Ok(_) => relation_type_manager
                .get(&ty)
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Relation type {} not found", ty))),
            Err(RelationTypeAddPropertyError::RelationTypeDoesNotExist(ty)) => {
                Err(Error::new(format!("Relation type {ty} does not exist")))
            }
            Err(RelationTypeAddPropertyError::AddPropertyError(AddPropertyError::PropertyAlreadyExist(property_name))) => {
                Err(Error::new(format!("Failed to add property to relation type {ty}: Property {property_name} already exists")))
            }
        }
    }

    /// Removes the property with the given property_name from the relation type with the given name.
    async fn remove_property(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        property_name: String,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        if !relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {ty} does not exist")));
        }
        if relation_type_manager.remove_property(&ty, property_name.as_str()).is_err() {
            return Err(Error::new(format!("Failed to remove property {property_name} from relation type {ty}")));
        }
        relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or_else(|| Error::new(format!("Relation type {ty} not found")))
    }

    /// Adds an extension to the relation type with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        match relation_type_manager.add_extension(&ty, extension.into()) {
            Ok(_) => relation_type_manager
                .get(&ty)
                .map(|relation_type| relation_type.into())
                .ok_or_else(|| Error::new(format!("Relation type {} not found", ty))),
            Err(RelationTypeAddExtensionError::RelationTypeDoesNotExist(_)) => {
                Err(Error::new(format!("Relation type {ty} does not exist")))
            }
            Err(RelationTypeAddExtensionError::AddExtensionError(AddExtensionError::ExtensionAlreadyExist(extension_ty))) => Err(Error::new(format!(
                "Failed to add extension {extension_ty} to relation type {ty}: Extension already exists"
            ))),
        }
    }

    // TODO: async fn update_extension() --> see flow_type mutation

    /// Removes the extension with the given extension_name from the relation type with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] relation_type: RelationTypeIdDefinition,
        #[graphql(name = "extension")] extension_ty: ExtensionTypeIdDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        if !relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {ty} does not exist")));
        }
        let extension_ty = extension_ty.into();
        if relation_type_manager.remove_extension(&ty, &extension_ty).is_err() {
            return Err(Error::new(format!("Failed to remove extension {extension_ty} from relation type {ty}")));
        }
        relation_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or_else(|| Error::new(format!("Relation type {ty} not found")))
    }

    /// Deletes the relation type with the given name.
    async fn delete(&self, context: &Context<'_>, #[graphql(name = "type")] relation_type: RelationTypeIdDefinition) -> Result<bool> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        Ok(relation_type_manager.delete(&relation_type.into()).is_some())
    }
}
