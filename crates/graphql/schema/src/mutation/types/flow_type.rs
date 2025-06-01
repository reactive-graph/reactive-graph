use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;
use uuid::Uuid;

use crate::mutation::ExtensionTypeIdDefinition;
use crate::mutation::FlowTypeIdDefinition;
use crate::mutation::GraphQLEntityInstanceDefinition;
use crate::mutation::GraphQLEntityInstanceDefinitions;
use crate::mutation::GraphQLRelationInstanceDefinition;
use crate::mutation::GraphQLRelationInstanceDefinitions;
use crate::mutation::PropertyTypeDefinition;
use crate::mutation::PropertyTypeDefinitions;
use crate::query::GraphQLExtension;
use crate::query::GraphQLExtensions;
use crate::query::GraphQLFlowType;
use reactive_graph_graph::Extension;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeAddEntityInstanceError;
use reactive_graph_graph::FlowTypeAddExtensionError;
use reactive_graph_graph::FlowTypeAddVariableError;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::FlowTypeRemoveEntityInstanceError;
use reactive_graph_graph::FlowTypeRemoveExtensionError;
use reactive_graph_graph::FlowTypeRemoveVariableError;
use reactive_graph_graph::FlowTypeUpdateEntityInstanceError;
use reactive_graph_graph::FlowTypeUpdateError;
use reactive_graph_graph::FlowTypeUpdateExtensionError;
use reactive_graph_graph::FlowTypeUpdateVariableError;
use reactive_graph_type_system_api::FlowTypeManager;

#[derive(Default)]
pub struct MutationFlowTypes;

/// Mutations for flow types
#[Object]
impl MutationFlowTypes {
    /// Creates a new flow type with the given name and components and properties.
    #[allow(clippy::too_many_arguments)]
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
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();

        let flow_type = FlowType::builder()
            .ty(&ty)
            .description(description.unwrap_or_default())
            .wrapper_entity_instance(wrapper_entity_instance)
            .entity_instances(GraphQLEntityInstanceDefinitions::new(entity_instances.unwrap_or_default()))
            .relation_instances(GraphQLRelationInstanceDefinitions::new(relation_instances.unwrap_or_default()))
            .variables(PropertyTypeDefinitions::new(variables.unwrap_or_default()))
            .extensions(GraphQLExtensions::new(extensions.unwrap_or_default()))
            .build();

        match flow_type_manager.register(flow_type.clone()) {
            Ok(flow_type) => Ok(flow_type.into()),
            Err(e) => Err(e.into()),
        }
    }

    /// Updates the description of the given flow type.
    async fn update_description(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] ty: FlowTypeIdDefinition,
        description: String,
    ) -> Result<GraphQLFlowType> {
        let relation_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = ty.into();
        match relation_type_manager.update_description(&ty, &description) {
            Ok(_) => relation_type_manager
                .get(&ty)
                .map(|relation_type| relation_type.into())
                .ok_or_else(|| Error::new(format!("Relation Type {ty} not found"))),
            Err(FlowTypeUpdateError::FlowTypeDoesNotExist(ty)) => {
                Err(Error::new(format!("Failed to update description of flow type {ty}: Flow type does not exist")))
            }
        }
    }

    /// Adds the given entity instance to the flow type with the given name.
    async fn add_entity_instance(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        entity_instance: GraphQLEntityInstanceDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();
        if let Err(e) = flow_type_manager.add_entity_instance(&ty, entity_instance.into()) {
            return Err(e.into());
        }
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeAddEntityInstanceError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the entity instance with the given id of the flow type with the given name.
    async fn update_entity_instance(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        id: Uuid,
        entity_instance: GraphQLEntityInstanceDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();
        if let Err(e) = flow_type_manager.update_entity_instance(&ty, id, entity_instance.into()) {
            return Err(e.into());
        }
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeUpdateEntityInstanceError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the entity instance with the given id from the flow type with the given name.
    async fn remove_entity_instance(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        id: Uuid,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();
        if let Err(e) = flow_type_manager.remove_entity_instance(&ty, id) {
            return Err(e.into());
        }
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeRemoveEntityInstanceError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Adds the given extension to the flow type with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();
        if let Err(e) = flow_type_manager.add_extension(&ty, extension.into()) {
            return Err(e.into());
        }
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeAddExtensionError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the extension with the given name of the flow type with the given name.
    async fn update_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        extension: GraphQLExtension,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();
        let extension: Extension = extension.into();
        if let Err(e) = flow_type_manager.update_extension(&ty, &extension.ty.clone(), extension) {
            return Err(e.into());
        }
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeUpdateExtensionError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the extension with the given name from the flow type with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        #[graphql(name = "extension", desc = "The extension type.")] extension_ty: ExtensionTypeIdDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();
        let extension_ty = extension_ty.into();
        if let Err(e) = flow_type_manager.remove_extension(&ty, &extension_ty) {
            return Err(e.into());
        }
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeRemoveExtensionError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Adds the given variable to the flow type with the given name.
    async fn add_variable(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        variable: PropertyTypeDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();
        if let Err(e) = flow_type_manager.add_variable(&ty, variable.into()) {
            return Err(e.into());
        }
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeAddVariableError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the variable with the given name of the flow type with the given name.
    async fn update_variable(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        variable_name: String,
        variable: PropertyTypeDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();
        if let Err(e) = flow_type_manager.update_variable(&ty, &variable_name, variable.into()) {
            return Err(e.into());
        }
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeUpdateVariableError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the variable with the given name from the flow type with the given name.
    async fn remove_variable(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition,
        variable_name: String,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();
        if let Err(e) = flow_type_manager.remove_variable(&ty, &variable_name) {
            return Err(e.into());
        }
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeRemoveVariableError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Deletes the flow type with the given name.
    async fn delete(&self, context: &Context<'_>, #[graphql(name = "type", desc = "The flow type.")] ty: FlowTypeIdDefinition) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty: FlowTypeId = ty.into();
        Ok(flow_type_manager.delete(&ty).is_some())
    }
}
