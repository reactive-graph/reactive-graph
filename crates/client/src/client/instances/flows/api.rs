use std::sync::Arc;

// use crate::client::instances::flows::mutations::create_from_type::mutations::create_flow_instance_from_type_mutation;
use crate::client::instances::flows::mutations::create_from_type::mutations::create_flow_instance_from_type_mutation;
use crate::client::instances::flows::mutations::delete::mutations::delete_flow_instance_mutation;
use crate::client::instances::flows::queries::get_by_id::queries::get_flow_instance_by_id;
use crate::client::instances::flows::queries::get_by_label::queries::get_flow_instance_by_label;
use crate::client::instances::flows::queries::search::queries::search;
use crate::client::instances::flows::variables::search::variables::SearchFlowInstancesVariables;
use crate::client::ReactiveGraphClient;
use crate::client::ReactiveGraphClientExecutionError;
use cynic::http::ReqwestExt;
use reactive_graph_graph::FlowInstance;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::PropertyInstances;
use uuid::Uuid;

pub struct FlowInstances {
    client: Arc<ReactiveGraphClient>,
}

impl FlowInstances {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn search(&self, search_query: SearchFlowInstancesVariables) -> Result<Option<Vec<FlowInstance>>, ReactiveGraphClientExecutionError> {
        let flow_instances = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(search(search_query))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| crate::schema_graphql::instances::flow_instance::FlowInstances(data.instances.flows))
            .map(From::from);
        Ok(flow_instances)
    }

    pub async fn get_by_id<ID: Into<Uuid>>(&self, id: ID) -> Result<Option<FlowInstance>, ReactiveGraphClientExecutionError> {
        let id = id.into();
        let flow_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_flow_instance_by_id(id))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.instances.flows.first().cloned())
            .map(From::from);
        Ok(flow_instance)
    }

    pub async fn get_by_label<L: Into<String>>(&self, label: L) -> Result<Option<FlowInstance>, ReactiveGraphClientExecutionError> {
        let label = label.into();
        let flow_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_flow_instance_by_label(label))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.instances.flows.first().cloned())
            .map(From::from);
        Ok(flow_instance)
    }

    // TODO: pub async fn create()

    pub async fn create_from_type<TY: Into<FlowTypeId>, ID: Into<Uuid>>(
        &self,
        ty: TY,
        id: Option<ID>,
        variables: PropertyInstances,
        properties: PropertyInstances,
    ) -> Result<Option<FlowInstance>, ReactiveGraphClientExecutionError> {
        let ty = ty.into();
        let id = id.map(|id| id.into());
        let flow_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create_flow_instance_from_type_mutation(ty, id, variables, properties))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.flows.create_from_type)
            .map(From::from);
        Ok(flow_instance)
    }

    pub async fn delete<ID: Into<Uuid>>(&self, id: ID) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let id = id.into();
        let flow_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete_flow_instance_mutation(id))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.flows.delete)
            .map(From::from);
        Ok(flow_instance)
    }
}
