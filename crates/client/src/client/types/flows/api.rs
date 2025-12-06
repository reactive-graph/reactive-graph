use std::sync::Arc;

use crate::client::ReactiveGraphClient;
use crate::client::ReactiveGraphClientExecutionError;
use crate::client::types::flows::mutations::add_extension::mutations::add_extension_mutation;
use crate::client::types::flows::mutations::add_variable::mutations::add_variable_mutation;
use crate::client::types::flows::mutations::create::mutations::create_flow_type_mutation;
use crate::client::types::flows::mutations::delete::mutations::delete_flow_type_mutation;
use crate::client::types::flows::mutations::remove_extension::mutations::remove_extension_mutation;
use crate::client::types::flows::mutations::remove_variable::mutations::remove_variable_mutation;
use crate::client::types::flows::mutations::update_description::mutations::update_description_mutation;
use crate::client::types::flows::queries::get_all::queries::get_all_flow_types_query;
use crate::client::types::flows::queries::get_by_type::queries::get_flow_type_by_type_query;
use crate::schema_graphql::types::flow_type::FlowTypes as FlowTypesVec;
use crate::types::flows::mutations::add_entity_instance::mutations::add_entity_instance_mutation;
use crate::types::flows::mutations::remove_entity_instance::mutations::remove_entity_instance_mutation;
use cynic::http::ReqwestExt;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::Extension;
use reactive_graph_graph::FlowExtensionTypeId;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::PropertyType;
use serde_json::Value;
use uuid::Uuid;

pub struct FlowTypes {
    client: Arc<ReactiveGraphClient>,
}

impl FlowTypes {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_flow_types(&self) -> Result<Option<reactive_graph_graph::FlowTypes>, ReactiveGraphClientExecutionError> {
        let Some(flow_types) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_all_flow_types_query())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| FlowTypesVec(data.types.flows))
        else {
            return Ok(None);
        };
        Ok(Some(
            reactive_graph_graph::FlowTypes::try_from(flow_types).map_err(|e| ReactiveGraphClientExecutionError::InvalidFlowType(e))?,
        ))
    }

    pub async fn get_flow_type_by_type<FT: Into<FlowTypeId>>(&self, ty: FT) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let Some(flow_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_flow_type_by_type_query(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.flows.first().cloned())
        else {
            return Ok(None);
        };
        Ok(Some(FlowType::try_from(flow_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidFlowType(e))?))
    }

    pub async fn json_schema_for_flow_type_by_type<FT: Into<FlowTypeId>>(&self, ty: FT) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let json_schema = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_flow_type_by_type_query(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.flows.first().cloned())
            .map(|flow_type| flow_type.json_schema);
        Ok(json_schema)
    }

    pub async fn create_flow_type(&self, flow_type: FlowType) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let Some(flow_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(create_flow_type_mutation(flow_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.create)
        else {
            return Ok(None);
        };
        Ok(Some(FlowType::try_from(flow_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidFlowType(e))?))
    }

    pub async fn delete_flow_type<FT: Into<FlowTypeId>>(&self, ty: FT) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let success = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(delete_flow_type_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.delete);
        Ok(success)
    }

    // ========

    pub async fn add_variable<FT: Into<FlowTypeId>, PT: Into<PropertyType>>(
        &self,
        ty: FT,
        property_type: PT,
    ) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let Some(flow_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_variable_mutation(ty, property_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.add_variable)
        else {
            return Ok(None);
        };
        Ok(Some(FlowType::try_from(flow_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidFlowType(e))?))
    }

    pub async fn remove_variable<FT: Into<FlowTypeId>>(&self, ty: FT, property_name: String) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let Some(flow_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_variable_mutation(ty, property_name))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.remove_variable)
        else {
            return Ok(None);
        };
        Ok(Some(FlowType::try_from(flow_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidFlowType(e))?))
    }

    // ========

    pub async fn add_extension<FT: Into<FlowTypeId>, EXT: Into<Extension>>(
        &self,
        ty: FT,
        extension: EXT,
    ) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let Some(flow_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_extension_mutation(ty, extension))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.add_extension)
        else {
            return Ok(None);
        };
        Ok(Some(FlowType::try_from(flow_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidFlowType(e))?))
    }

    pub async fn remove_extension<FE: Into<FlowExtensionTypeId>>(&self, flow_extension_ty: FE) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let Some(flow_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_extension_mutation(flow_extension_ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.remove_extension)
        else {
            return Ok(None);
        };
        Ok(Some(FlowType::try_from(flow_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidFlowType(e))?))
    }

    // ========

    pub async fn add_entity_instance<FT: Into<FlowTypeId>, EI: Into<EntityInstance>>(
        &self,
        ty: FT,
        entity_instance: EI,
    ) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let Some(flow_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_entity_instance_mutation(ty, entity_instance))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.add_entity_instance)
        else {
            return Ok(None);
        };
        Ok(Some(FlowType::try_from(flow_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidFlowType(e))?))
    }

    pub async fn remove_entity_instance<FT: Into<FlowTypeId>, ID: Into<Uuid>>(
        &self,
        ty: FT,
        id: ID,
    ) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let Some(flow_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_entity_instance_mutation(ty, id))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.remove_entity_instance)
        else {
            return Ok(None);
        };
        Ok(Some(FlowType::try_from(flow_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidFlowType(e))?))
    }

    // ========

    pub async fn update_description<FT: Into<FlowTypeId>>(&self, ty: FT, description: String) -> Result<Option<FlowType>, ReactiveGraphClientExecutionError> {
        let Some(flow_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(update_description_mutation(ty, description))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.flows.update_description)
        else {
            return Ok(None);
        };
        Ok(Some(FlowType::try_from(flow_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidFlowType(e))?))
    }
}
