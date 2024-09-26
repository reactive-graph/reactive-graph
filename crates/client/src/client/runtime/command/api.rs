use std::sync::Arc;

use cynic::http::ReqwestExt;
use serde_json::Value;

use crate::client::runtime::command::mutations::execute_command::mutations::execute_command;
use crate::ReactiveGraphClient;
use crate::ReactiveGraphClientExecutionError;

pub struct Command {
    client: Arc<ReactiveGraphClient>,
}

impl Command {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn execute(&self, name: String, args: Option<Value>) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let value = self
            .client
            .client
            .post(self.client.url_runtime())
            .run_graphql(execute_command(name, args))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.commands.execute)
            .map(|property_instance| property_instance.value.0);
        Ok(value)
    }
}
