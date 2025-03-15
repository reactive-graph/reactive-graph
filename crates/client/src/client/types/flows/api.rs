use std::sync::Arc;

use crate::client::types::extensions::container::queries::ExtensionContainerVariables;
use crate::client::types::flows::add_extension::queries::add_extension_mutation;
use crate::client::types::flows::add_extension::queries::add_extension_with_variables;
use crate::client::types::flows::add_extension::queries::AddExtensionVariables;
use crate::client::types::flows::add_variable::queries::add_variable_mutation;
use crate::client::types::flows::add_variable::queries::add_variable_with_variables;
use crate::client::types::flows::add_variable::queries::AddVariableVariables;
use crate::client::types::flows::create::queries::create_flow_type_mutation;
use crate::client::types::flows::create::queries::create_flow_type_with_variables;
use crate::client::types::flows::create::queries::CreateFlowTypeVariables;
use crate::client::types::flows::delete::queries::delete_flow_type_mutation;
use crate::client::types::flows::delete::queries::delete_flow_type_with_variables;
use crate::client::types::flows::get_all::queries::get_all_flow_types_query;
use crate::client::types::flows::get_by_type::queries::get_flow_type_by_type_query;
use crate::client::types::flows::remove_extension::queries::remove_extension_mutation;
use crate::client::types::flows::remove_extension::queries::remove_extension_with_variables;
use crate::client::types::flows::remove_variable::queries::remove_variable_mutation;
use crate::client::types::flows::remove_variable::queries::remove_variable_with_variables;
use crate::client::types::flows::type_id::queries::FlowTypeIdVariables;
use crate::client::types::flows::update_description::queries::update_description_mutation;
use crate::client::types::flows::update_description::queries::update_description_with_variables;
use crate::client::types::flows::update_description::queries::UpdateDescriptionVariables;
use crate::client::types::properties::container::queries::PropertyContainerVariables;
use crate::client::ReactiveGraphClient;
use crate::client::ReactiveGraphClientExecutionError;
use crate::schema_graphql::types::flow_type::FlowTypes as FlowTypesVec;
use cynic::http::ReqwestExt;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeId;

pub struct FlowTypes {
    client: Arc<ReactiveGraphClient>,
}

impl FlowTypes {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_flow_types(&self) -> Result<Option<Vec<FlowType>>, ReactiveGraphClientExecutionError> {
        let flow_types = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_all_flow_types_query())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| FlowTypesVec(data.types.flows))
            .map(From::from);
        Ok(flow_types)
    }

    pub async fn get_flow_type_by_type<C: Into<FlowTypeId>>(&self, ty: C) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let ty = ty.into();
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_flow_type_by_type_query(&ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.flows.first().cloned())
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn create_flow_type(&self, flow_type: FlowType) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create_flow_type_mutation(flow_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.create)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn create_flow_type_with_variables(&self, variables: CreateFlowTypeVariables) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create_flow_type_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.create)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn delete_flow_type<C: Into<FlowTypeId>>(&self, ty: C) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let ty: FlowTypeId = ty.into();
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete_flow_type_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.delete)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn delete_flow_type_with_variables(&self, variables: FlowTypeIdVariables) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete_flow_type_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.delete)
            .map(From::from);
        Ok(flow_type)
    }

    // ========

    pub async fn add_variable(
        &self,
        ty: FlowTypeId,
        property_type: reactive_graph_graph::PropertyType,
    ) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_variable_mutation(ty, property_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.add_variable)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn add_variable_with_variables(&self, variables: AddVariableVariables) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_variable_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.add_variable)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn remove_variable(&self, ty: FlowTypeId, property_name: String) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_variable_mutation(ty, property_name))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.remove_variable)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn remove_variable_with_variables(&self, variables: PropertyContainerVariables) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_variable_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.remove_variable)
            .map(From::from);
        Ok(flow_type)
    }

    // ========

    pub async fn add_extension(
        &self,
        ty: FlowTypeId,
        extension: reactive_graph_graph::Extension,
    ) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_extension_mutation(ty, extension))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.add_extension)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn add_extension_with_variables(&self, variables: AddExtensionVariables) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_extension_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.add_extension)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn remove_extension(&self, ty: FlowTypeId, extension_ty: ExtensionTypeId) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_extension_mutation(ty, extension_ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.remove_extension)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn remove_extension_with_variables(&self, variables: ExtensionContainerVariables) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_extension_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.remove_extension)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn update_description(&self, ty: FlowTypeId, description: String) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let flow_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(update_description_mutation(ty, description))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.update_description)
            .map(From::from);
        Ok(flow_type)
    }

    pub async fn update_description_with_variables(
        &self,
        variables: UpdateDescriptionVariables,
    ) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(update_description_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.update_description)
            .map(From::from);
        Ok(component)
    }
}
