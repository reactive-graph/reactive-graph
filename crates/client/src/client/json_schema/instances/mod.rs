pub mod entities;
pub mod flows;
pub mod relations;

use crate::ReactiveGraphClientExecutionError;
use crate::client::ReactiveGraphClient;
use crate::client::json_schema::instances::entities::queries::get_json_schema_for_entity_instances;
use crate::client::json_schema::instances::flows::queries::get_json_schema_for_flow_instances;
use crate::client::json_schema::instances::relations::queries::get_json_schema_for_relation_instances;
use cynic::http::ReqwestExt;
use serde_json::Value;
use std::sync::Arc;

pub struct JsonSchemaInstanceSystem {
    client: Arc<ReactiveGraphClient>,
}

impl JsonSchemaInstanceSystem {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn entities(&self) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let json_schema = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_json_schema_for_entity_instances())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.json_schema.instances.entities);
        Ok(json_schema)
    }

    pub async fn relations(&self) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let json_schema = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_json_schema_for_relation_instances())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.json_schema.instances.relations);
        Ok(json_schema)
    }

    pub async fn flows(&self) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let json_schema = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_json_schema_for_flow_instances())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.json_schema.instances.flows);
        Ok(json_schema)
    }
}
