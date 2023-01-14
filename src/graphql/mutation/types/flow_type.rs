use std::sync::Arc;

use async_graphql::*;
use log::debug;
use uuid::Uuid;

use crate::api::FlowTypeManager;
use crate::builder::FlowTypeBuilder;
use crate::graphql::mutation::ExtensionTypeIdDefinition;
use crate::graphql::mutation::FlowTypeIdDefinition;
use crate::graphql::mutation::GraphQLEntityInstanceDefinition;
use crate::graphql::mutation::GraphQLRelationInstanceDefinition;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::GraphQLFlowType;
use crate::model::FlowTypeId;

#[derive(Default)]
pub struct MutationFlowTypes;

/// Mutations for flow types
#[Object]
impl MutationFlowTypes {
    /// Creates a new flow type with the given name and components and properties.
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        #[graphql(desc = "Textual description of the flow type.")] description: Option<String>,
        wrapper_entity_instance: GraphQLEntityInstanceDefinition,
        entity_instances: Option<Vec<GraphQLEntityInstanceDefinition>>,
        relation_instances: Option<Vec<GraphQLRelationInstanceDefinition>>,
        #[graphql(desc = "The variables of the flow type.")] variables: Option<Vec<PropertyTypeDefinition>>,
        #[graphql(desc = "The extension on the entity type.")] extensions: Option<Vec<GraphQLExtension>>,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();

        if flow_type_manager.has(&ty) {
            return Err(Error::new(format!("Flow type {} already exists", &ty)));
        }

        let mut flow_type_builder = FlowTypeBuilder::new(&ty, wrapper_entity_instance.into());
        if let Some(description) = description {
            flow_type_builder.description(description);
        }
        if let Some(entity_instances) = entity_instances {
            for entity_instance in entity_instances {
                flow_type_builder.entity_instance(entity_instance.into());
            }
        }
        if let Some(relation_instances) = relation_instances {
            for relation_instance in relation_instances {
                flow_type_builder.relation_instance(relation_instance.into());
            }
        }
        if let Some(variables) = variables {
            for variable in variables {
                debug!("{} {}", variable.name, variable.data_type.to_string());
                flow_type_builder.variable(variable.name, variable.data_type.into());
            }
        }
        if let Some(extensions) = extensions {
            for extension in extensions {
                debug!("{} {}", &extension.ty, extension.extension.to_string());
                flow_type_builder.extension(extension.ty.namespace, extension.ty.type_name, extension.extension.clone());
            }
        }

        let flow_type = flow_type_builder.build();
        flow_type_manager.register(flow_type.clone());
        Ok(flow_type.into())
    }

    /// Adds the given entity instance to the flow type with the given name.
    async fn add_entity_instance(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        entity_instance: GraphQLEntityInstanceDefinition,
    ) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();

        if flow_type_manager.has(&ty) {
            return Err(Error::new(format!("Flow type {} already exists", &ty)));
        }
        flow_type_manager.add_entity_instance(&ty, entity_instance.into());
        Ok(true)
    }

    /// Updates the entity instance with the given id of the flow type with the given name.
    async fn update_entity_instance(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        id: Uuid,
        entity_instance: GraphQLEntityInstanceDefinition,
    ) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();
        if !flow_type_manager.has(&ty) {
            return Err(Error::new(format!("Flow type {} does not exist", &ty)));
        }
        flow_type_manager.update_entity_instance(&ty, id, entity_instance.into());
        Ok(true)
    }

    /// Removes the entity instance with the given id from the flow type with the given name.
    async fn remove_entity_instance(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        id: Uuid,
    ) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();
        if !flow_type_manager.has(&ty) {
            return Err(Error::new(format!("Flow type {} does not exist", &ty)));
        }
        flow_type_manager.remove_entity_instance(&ty, id);
        Ok(true)
    }

    /// Adds the given extension to the flow type with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();
        if !flow_type_manager.has(&ty) {
            return Err(Error::new(format!("Flow type {} does not exist", &ty)));
        }
        flow_type_manager.add_extension(&ty, extension.into());
        Ok(true)
    }

    /// Updates the extension with the given name of the flow type with the given name.
    async fn update_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();
        if !flow_type_manager.has(&ty) {
            return Err(Error::new(format!("Flow type {} does not exist", &ty)));
        }
        flow_type_manager.update_extension(&ty, extension.into());
        Ok(true)
    }

    /// Removes the extension with the given name from the flow type with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        #[graphql(name = "extension", desc = "The extension type.")] extension_ty: ExtensionTypeIdDefinition,
    ) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();
        if !flow_type_manager.has(&ty) {
            return Err(Error::new(format!("Flow type {} does not exist", &ty)));
        }
        let extension_ty = extension_ty.into();
        flow_type_manager.remove_extension(&ty, &extension_ty);
        Ok(true)
    }

    /// Adds the given variable to the flow type with the given name.
    async fn add_variable(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        variable: PropertyTypeDefinition,
    ) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();
        if !flow_type_manager.has(&ty) {
            return Err(Error::new(format!("Flow type {} does not exist", &ty)));
        }
        flow_type_manager.add_variable(&ty, variable.into());
        Ok(true)
    }

    /// Updates the variable with the given name of the flow type with the given name.
    async fn update_variable(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        variable_name: String,
        variable: PropertyTypeDefinition,
    ) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();
        if !flow_type_manager.has(&ty) {
            return Err(Error::new(format!("Flow type {} does not exist", &ty)));
        }
        flow_type_manager.update_variable(&ty, &variable_name, variable.into());
        Ok(true)
    }

    /// Removes the variable with the given name from the flow type with the given name.
    async fn remove_variable(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        variable_name: String,
    ) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();
        if !flow_type_manager.has(&ty) {
            return Err(Error::new(format!("Flow type {} does not exist", &ty)));
        }
        flow_type_manager.remove_variable(&ty, &variable_name);
        Ok(true)
    }

    /// Deletes the flow type with the given name.
    async fn delete(&self, context: &Context<'_>, #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let ty: FlowTypeId = ty.into();
        flow_type_manager.delete(&ty);
        Ok(true)
    }
}
