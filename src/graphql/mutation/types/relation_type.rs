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
use crate::graphql::mutation::PropertyTypeDefinition;
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
        #[graphql(desc = "The namespace of the relation type.")] namespace: Option<String>,
        outbound_type: String,
        #[graphql(desc = "The name of the relation type.")] name: String,
        inbound_type: String,
        #[graphql(desc = "Adds the given components to the newly created relation type.")] components: Option<Vec<String>>,
        #[graphql(desc = "The definitions of properties. These are added additionally to the properties provided by the given components.")] properties: Option<
            Vec<PropertyTypeDefinition>,
        >,
        #[graphql(desc = "The extension on the relation type.")] extensions: Option<Vec<GraphQLExtension>>,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;

        if relation_type_manager.has(&name) {
            return Err(Error::new(format!("Relation type {} already exists", name)));
        }
        if !entity_type_manager.has(&outbound_type) {
            return Err(Error::new(format!("Outbound entity type {} does not exist", outbound_type)));
        }
        if !entity_type_manager.has(&inbound_type) {
            return Err(Error::new(format!("Inbound entity type {} does not exist", inbound_type)));
        }

        let namespace = namespace.unwrap_or_default();
        let mut relation_type_builder = RelationTypeBuilder::new(namespace, outbound_type, name, inbound_type);
        if components.is_some() {
            for component in components.unwrap() {
                debug!("Add component {}", component.clone());
                relation_type_builder.component(component.clone());
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
            Err(RelationTypeRegistrationError::RelationTypeAlreadyExists(namespace, name)) => {
                Err(Error::new(format!("Failed to create relation type {}__{}: relation type already exists", namespace, name)))
            }
            Err(RelationTypeRegistrationError::OutboundEntityTypeDoesNotExist(namespace, name, outbound_type)) => Err(Error::new(format!(
                "Failed to create relation type {}__{}: outbound entity type {} does not exist",
                namespace, name, outbound_type
            ))),
            Err(RelationTypeRegistrationError::InboundEntityTypeDoesNotExist(namespace, name, inbound_type)) => Err(Error::new(format!(
                "Failed to create relation type {}__{}: inbound entity type {} does not exist",
                namespace, name, inbound_type
            ))),
        }
    }

    /// Adds the component with the given component_name to the relation type with the given name.
    async fn add_component(&self, context: &Context<'_>, name: String, component_name: String) -> Result<GraphQLRelationType> {
        if let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager>>() {
            if !relation_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Relation type {} does not exist", name)));
            }
            return match relation_type_manager.add_component(name.as_str(), component_name.as_str()) {
                Ok(_) => relation_type_manager
                    .get(name.as_str())
                    .map(|relation_type| relation_type.into())
                    .ok_or_else(|| Error::new(format!("Relation type {} not found", name))),
                Err(RelationTypeComponentError::ComponentAlreadyAssigned) => {
                    Err(Error::new(format!("Relation type {} has already component {}", name, component_name)))
                }
                Err(RelationTypeComponentError::ComponentDoesNotExist) => Err(Error::new(format!("Component {} doesn't exist", component_name))),
            };
        }
        Err(Error::new(format!("Failed to add component {} to relation type {}", component_name, name)))
    }

    /// Remove the component with the given component_name from the relation type with the given name.
    async fn remove_component(&self, context: &Context<'_>, name: String, component_name: String) -> Result<GraphQLRelationType> {
        if let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager>>() {
            if !relation_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Relation type {} does not exist", name)));
            }
            relation_type_manager.remove_component(name.as_str(), component_name.as_str());
            return relation_type_manager
                .get(name.as_str())
                .map(|relation_type| relation_type.into())
                .ok_or_else(|| Error::new(format!("Relation type {} not found", name)));
        }
        Err(Error::new(format!("Failed to add component {} to relation type {}", component_name, name)))
    }

    /// Adds a property to the relation type with the given name.
    async fn add_property(&self, context: &Context<'_>, name: String, property: PropertyTypeDefinition) -> Result<GraphQLRelationType> {
        if let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager>>() {
            if !relation_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Relation type {} does not exist", name)));
            }
            return match relation_type_manager.add_property(name.as_str(), property.into()) {
                Ok(_) => relation_type_manager
                    .get(name.as_str())
                    .map(|entity_type| entity_type.into())
                    .ok_or_else(|| Error::new(format!("Entity type {} not found", name))),
                Err(RelationTypePropertyError::PropertyAlreadyExists) => {
                    Err(Error::new(format!("Failed to add property to relation type {}: Property already exists", name)))
                }
            };
        }
        Err(Error::new(format!("Failed to add property to relation type {}", name)))
    }

    /// Removes the property with the given property_name from the relation type with the given name.
    async fn remove_property(&self, context: &Context<'_>, name: String, property_name: String) -> Result<GraphQLRelationType> {
        if let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager>>() {
            if !relation_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Relation type {} does not exist", name)));
            }
            relation_type_manager.remove_property(name.as_str(), property_name.as_str());
            return relation_type_manager
                .get(name.as_str())
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Relation type {} not found", name)));
        }
        Err(Error::new(format!("Failed to remove property {} from relation type {}", property_name, name)))
    }

    /// Adds an extension to the relation type with the given name.
    async fn add_extension(&self, context: &Context<'_>, name: String, extension: GraphQLExtension) -> Result<GraphQLRelationType> {
        if let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager>>() {
            if !relation_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Relation type {} does not exist", name)));
            }
            return match relation_type_manager.add_extension(name.as_str(), extension.into()) {
                Ok(_) => relation_type_manager
                    .get(name.as_str())
                    .map(|entity_type| entity_type.into())
                    .ok_or_else(|| Error::new(format!("Entity type {} not found", name))),
                Err(RelationTypeExtensionError::ExtensionAlreadyExists) => {
                    Err(Error::new(format!("Failed to add extension to relation type {}: Extension already exists", name)))
                }
            };
        }
        Err(Error::new(format!("Failed to add extension to relation type {}", name)))
    }

    /// Removes the extension with the given extension_name from the relation type with the given name.
    async fn remove_extension(&self, context: &Context<'_>, name: String, extension_name: String) -> Result<GraphQLRelationType> {
        if let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager>>() {
            if !relation_type_manager.has(name.as_str()) {
                return Err(Error::new(format!("Relation type {} does not exist", name)));
            }
            relation_type_manager.remove_extension(name.as_str(), extension_name.as_str());
            return relation_type_manager
                .get(name.as_str())
                .map(|entity_type| entity_type.into())
                .ok_or_else(|| Error::new(format!("Relation type {} not found", name)));
        }
        Err(Error::new(format!("Failed to remove extension {} from relation type {}", extension_name, name)))
    }

    /// Deletes the relation type with the given name.
    async fn delete(&self, context: &Context<'_>, name: String) -> Result<bool> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        relation_type_manager.delete(name.as_str());
        Ok(true)
    }
}
