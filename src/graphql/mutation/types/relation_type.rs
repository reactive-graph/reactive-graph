use std::sync::Arc;

use async_graphql::*;
use log::debug;

use crate::api::EntityTypeManager;
use crate::api::RelationTypeComponentError;
use crate::api::RelationTypeExtensionError;
use crate::api::RelationTypeManager;
use crate::api::RelationTypePropertyError;
use crate::api::RelationTypeRegistrationError;
use crate::builder::RelationTypeBuilder;
use crate::graphql::mutation::ComponentTypeIdDefinition;
use crate::graphql::mutation::EntityTypeIdDefinition;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::mutation::RelationTypeIdDefinition;
use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::GraphQLRelationType;

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

        let mut relation_type_builder = RelationTypeBuilder::new(outbound_ty.clone(), &ty, inbound_ty.clone());
        if components.is_some() {
            for component in components.unwrap() {
                debug!("Add component {}", &component);
                relation_type_builder.component(component);
            }
        }
        if properties.is_some() {
            for property in properties.unwrap() {
                debug!("Add property {} {} {}", property.name, property.data_type.to_string(), property.socket_type.to_string());
                relation_type_builder.property_from(property.clone());
            }
        }
        if extensions.is_some() {
            for extension in extensions.unwrap() {
                debug!("{} {}", extension.name, extension.extension.to_string());
                relation_type_builder.extension(extension.name, extension.extension.clone());
            }
        }

        let relation_type = relation_type_builder.build();
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
        relation_type: RelationTypeIdDefinition,
        component: ComponentTypeIdDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        let component_ty = component.into();
        if !relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {} does not exist", ty)));
        }
        return match relation_type_manager.add_component(&ty, &component_ty) {
            Ok(_) => relation_type_manager
                .get(&ty)
                .map(|relation_type| relation_type.into())
                .ok_or_else(|| Error::new(format!("Relation type {} not found", ty))),
            Err(RelationTypeComponentError::ComponentAlreadyAssigned) => {
                Err(Error::new(format!("Relation type {} has already component {}", ty, component_ty)))
            }
            Err(RelationTypeComponentError::ComponentDoesNotExist) => Err(Error::new(format!("Component {} doesn't exist", component_ty))),
        };
    }

    /// Remove the component with the given component_name from the relation type with the given name.
    async fn remove_component(
        &self,
        context: &Context<'_>,
        relation_type: RelationTypeIdDefinition,
        component: ComponentTypeIdDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        let component_ty = component.into();
        if !relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {} does not exist", ty)));
        }
        relation_type_manager.remove_component(&ty, &component_ty);
        return relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or_else(|| Error::new(format!("Relation type {} not found", ty)));
    }

    /// Adds a property to the relation type with the given name.
    async fn add_property(
        &self,
        context: &Context<'_>,
        relation_type: RelationTypeIdDefinition,
        property: PropertyTypeDefinition,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        if !relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {} does not exist", ty)));
        }
        return match relation_type_manager.add_property(&ty, property.into()) {
            Ok(_) => relation_type_manager
                .get(&ty)
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Relation type {} not found", ty))),
            Err(RelationTypePropertyError::PropertyAlreadyExists) => {
                Err(Error::new(format!("Failed to add property to relation type {}: Property already exists", ty)))
            }
        };
    }

    /// Removes the property with the given property_name from the relation type with the given name.
    async fn remove_property(&self, context: &Context<'_>, relation_type: RelationTypeIdDefinition, property_name: String) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        if !relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {} does not exist", ty)));
        }
        relation_type_manager.remove_property(&ty, property_name.as_str());
        return relation_type_manager
            .get(&ty)
            .map(|relation_type| relation_type.into())
            .ok_or_else(|| Error::new(format!("Relation type {} not found", ty)));
    }

    /// Adds an extension to the relation type with the given name.
    async fn add_extension(&self, context: &Context<'_>, relation_type: RelationTypeIdDefinition, extension: GraphQLExtension) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        if !relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {} does not exist", ty)));
        }
        return match relation_type_manager.add_extension(&ty, extension.into()) {
            Ok(_) => relation_type_manager
                .get(&ty)
                .map(|relation_type| relation_type.into())
                .ok_or_else(|| Error::new(format!("Relation type {} not found", ty))),
            Err(RelationTypeExtensionError::ExtensionAlreadyExists) => {
                Err(Error::new(format!("Failed to add extension to relation type {}: Extension already exists", ty)))
            }
        };
    }

    /// Removes the extension with the given extension_name from the relation type with the given name.
    async fn remove_extension(&self, context: &Context<'_>, relation_type: RelationTypeIdDefinition, extension_name: String) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let ty = relation_type.into();
        if !relation_type_manager.has(&ty) {
            return Err(Error::new(format!("Relation type {} does not exist", ty)));
        }
        relation_type_manager.remove_extension(&ty, extension_name.as_str());
        return relation_type_manager
            .get(&ty)
            .map(|entity_type| entity_type.into())
            .ok_or_else(|| Error::new(format!("Relation type {} not found", ty)));
    }

    /// Deletes the relation type with the given name.
    async fn delete(&self, context: &Context<'_>, relation_type: RelationTypeIdDefinition) -> Result<bool> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        relation_type_manager.delete(&relation_type.into());
        Ok(true)
    }
}
