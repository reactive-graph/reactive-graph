use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use uuid::Uuid;

use crate::mutation::GraphQLEntityInstanceDefinition;
use crate::mutation::GraphQLEntityInstanceDefinitions;
use crate::mutation::GraphQLExtensionDefinition;
use crate::mutation::GraphQLExtensionDefinitions;
use crate::mutation::GraphQLRelationInstanceDefinition;
use crate::mutation::GraphQLRelationInstanceDefinitions;
use crate::mutation::PropertyTypeDefinition;
use crate::mutation::PropertyTypeDefinitions;
use crate::query::GraphQLFlowType;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeAddEntityInstanceError;
use reactive_graph_graph::FlowTypeAddExtensionError;
use reactive_graph_graph::FlowTypeAddVariableError;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::FlowTypeRemoveEntityInstanceError;
use reactive_graph_graph::FlowTypeRemoveExtensionError;
use reactive_graph_graph::FlowTypeRemoveVariableError;
use reactive_graph_graph::FlowTypeUpdateEntityInstanceError;
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
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
        #[graphql(desc = "Textual description of the flow type.")] description: Option<String>,
        wrapper_entity_instance: GraphQLEntityInstanceDefinition,
        entity_instances: Option<Vec<GraphQLEntityInstanceDefinition>>,
        relation_instances: Option<Vec<GraphQLRelationInstanceDefinition>>,
        #[graphql(desc = "The variables of the flow type.")] variables: Option<Vec<PropertyTypeDefinition>>,
        #[graphql(desc = "The extensions of the entity type.")] extensions: Option<Vec<GraphQLExtensionDefinition>>,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        let variables = PropertyTypeDefinitions::parse_optional_definitions(variables)?;
        let extensions = GraphQLExtensionDefinitions::parse_optional_definitions(extensions)?;
        let flow_type = FlowType::builder()
            .ty(&ty)
            .description(description.unwrap_or_default())
            .wrapper_entity_instance(wrapper_entity_instance)
            .entity_instances(GraphQLEntityInstanceDefinitions::new(entity_instances.unwrap_or_default()))
            .relation_instances(GraphQLRelationInstanceDefinitions::new(relation_instances.unwrap_or_default()))
            .variables(variables)
            .extensions(extensions)
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
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
        description: String,
    ) -> Result<GraphQLFlowType> {
        let relation_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        Ok(relation_type_manager.update_description(&ty, &description)?.into())
    }

    /// Adds the given entity instance to the flow type with the given name.
    async fn add_entity_instance(
        &self,
        context: &Context<'_>,
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
        entity_instance: GraphQLEntityInstanceDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        flow_type_manager.add_entity_instance(&ty, entity_instance.into())?;
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeAddEntityInstanceError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the entity instance with the given id of the flow type with the given name.
    async fn update_entity_instance(
        &self,
        context: &Context<'_>,
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
        id: Uuid,
        entity_instance: GraphQLEntityInstanceDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        flow_type_manager.update_entity_instance(&ty, id, entity_instance.into())?;
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeUpdateEntityInstanceError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the entity instance with the given id from the flow type with the given name.
    async fn remove_entity_instance(
        &self,
        context: &Context<'_>,
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
        id: Uuid,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        flow_type_manager.remove_entity_instance(&ty, id)?;
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeRemoveEntityInstanceError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Adds the given extension to the flow type with the given name.
    async fn add_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
        extension: GraphQLExtensionDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        let extension = extension.try_into()?;
        flow_type_manager.add_extension(&ty, extension)?;
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeAddExtensionError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the extension with the given name of the flow type with the given name.
    async fn update_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
        extension: GraphQLExtensionDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        let extension: Extension = extension.try_into()?;
        // let extension: Extension = extension.into();
        flow_type_manager.update_extension(&ty, &extension.ty.clone(), extension)?;
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeUpdateExtensionError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the extension with the given name from the flow type with the given name.
    async fn remove_extension(
        &self,
        context: &Context<'_>,
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] flow_namespace: String,
        #[graphql(name = "extension", desc = "The fully qualified namespace of the extension.")] extension_namespace: String,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let flow_ty = FlowTypeId::parse_namespace(&flow_namespace)?;
        let extension_ty = ExtensionTypeId::parse_namespace(&extension_namespace)?;
        flow_type_manager.remove_extension(&flow_ty, &extension_ty)?;
        flow_type_manager
            .get(&flow_ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeRemoveExtensionError::FlowTypeDoesNotExist(flow_ty.clone()).into())
    }

    /// Adds the given variable to the flow type with the given name.
    async fn add_variable(
        &self,
        context: &Context<'_>,
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
        variable: PropertyTypeDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        let variable = variable.try_into()?;
        flow_type_manager.add_variable(&ty, variable)?;
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeAddVariableError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Updates the variable with the given name of the flow type with the given name.
    async fn update_variable(
        &self,
        context: &Context<'_>,
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
        variable_name: String,
        variable: PropertyTypeDefinition,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        let variable = variable.try_into()?;
        flow_type_manager.update_variable(&ty, &variable_name, variable)?;
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeUpdateVariableError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Removes the variable with the given name from the flow type with the given name.
    async fn remove_variable(
        &self,
        context: &Context<'_>,
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
        variable_name: String,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        flow_type_manager.remove_variable(&ty, &variable_name)?;
        flow_type_manager
            .get(&ty)
            .map(|flow_type| flow_type.into())
            .ok_or(FlowTypeRemoveVariableError::FlowTypeDoesNotExist(ty.clone()).into())
    }

    /// Deletes the flow type with the given name.
    async fn delete(
        &self,
        context: &Context<'_>,
        #[graphql(name = "name", desc = "The fully qualified namespace of the flow type.")] namespace: String,
    ) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let ty = FlowTypeId::parse_namespace(&namespace)?;
        Ok(flow_type_manager.delete(&ty).is_some())
    }
}
