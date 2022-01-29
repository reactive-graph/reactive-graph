use std::sync::Arc;

use async_graphql::*;
use log::debug;

use crate::api::{EntityTypeManager, RelationTypeManager};
use crate::builder::RelationTypeBuilder;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::query::{GraphQLExtension, GraphQLRelationType};

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
        outbound_type: String,
        #[graphql(desc = "The name of the entity type.")] name: String,
        inbound_type: String,
        #[graphql(desc = "Adds the given components to the newly created relation type.")] components: Option<Vec<String>>,
        behaviours: Option<Vec<String>>,
        #[graphql(desc = "The definitions of properties. These are added additionally to the properties provided by the given components.")] properties: Option<
            Vec<PropertyTypeDefinition>,
        >,
        #[graphql(desc = "The extension on the relation type.")] extensions: Option<Vec<GraphQLExtension>>,
    ) -> Result<GraphQLRelationType> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;

        if relation_type_manager.has(name.clone()) {
            return Err(Error::new(format!("Relation type {} already exists", name)));
        }
        if !entity_type_manager.has(outbound_type.clone()) {
            return Err(Error::new(format!("Outbound entity type {} does not exist", outbound_type)));
        }
        if !entity_type_manager.has(inbound_type.clone()) {
            return Err(Error::new(format!("Inbound entity type {} does not exist", inbound_type)));
        }

        let mut relation_type_builder = RelationTypeBuilder::new(outbound_type, name, inbound_type);
        if components.is_some() {
            for component in components.unwrap() {
                debug!("Add component {}", component.clone());
                relation_type_builder.component(component.clone());
            }
        }
        if behaviours.is_some() {
            for behaviour in behaviours.unwrap() {
                debug!("Add behaviour {}", behaviour.clone());
                relation_type_builder.behaviour(behaviour.clone());
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
        relation_type_manager.register(relation_type.clone());
        Ok(relation_type.into())
    }

    // async fn add_component(
    //     &self,
    //     context: &Context<'_>,
    //     name: String,
    //     component: String
    // ) -> Result<GraphQLRelationType> {
    //     let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
    //     let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
    //     let relation_type = relation_type_manager.get(name.clone());
    //     if relation_type.is_none() {
    //         return Err(Error::new(format!("Relation type {} does not exist", name.clone())));
    //     }
    //     let component = component_manager.get(component.clone());
    //     if component.is_none() {
    //         return Err(Error::new(format!("Component {} does not exist", component.clone())));
    //     }
    //     let component = component.unwrap();
    //
    //     let mut relation_type = relation_type.unwrap();
    //     relation_type.components.push(component.name.clone());
    //     Ok(relation_type.into())
    // }

    // TODO: add component
    // TODO: remove component
    // TODO: add behaviour
    // TODO: remove behaviour
    // TODO: add property
    // TODO: remove property

    /// Deletes the relation type with the given name.
    async fn delete(&self, context: &Context<'_>, name: String) -> Result<bool> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        relation_type_manager.delete(name);
        Ok(true)
    }
}
