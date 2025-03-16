pub mod components;
pub mod entities;
pub mod flows;
pub mod relations;

use crate::ReactiveGraphClientExecutionError;
use crate::client::ReactiveGraphClient;
use crate::client::json_schema::types::components::queries::get_json_schema_for_components;
use crate::client::json_schema::types::entities::queries::get_json_schema_for_entity_types;
use crate::client::json_schema::types::flows::queries::get_json_schema_for_flow_types;
use crate::client::json_schema::types::relations::queries::get_json_schema_for_relation_types;
use cynic::http::ReqwestExt;
use serde_json::Value;
use std::sync::Arc;

pub struct JsonSchemaTypeSystem {
    client: Arc<ReactiveGraphClient>,
}

impl JsonSchemaTypeSystem {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn components(&self) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let json_schema = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_json_schema_for_components())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.json_schema.types.components)
            .map(From::from);
        Ok(json_schema)
    }

    pub async fn entities(&self) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let json_schema = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_json_schema_for_entity_types())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.json_schema.types.entities)
            .map(From::from);
        Ok(json_schema)
    }

    pub async fn relations(&self) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let json_schema = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_json_schema_for_relation_types())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.json_schema.types.relations)
            .map(From::from);
        Ok(json_schema)
    }

    pub async fn flows(&self) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let json_schema = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_json_schema_for_flow_types())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.json_schema.types.flows)
            .map(From::from);
        Ok(json_schema)
    }
}
