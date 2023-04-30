use std::sync::Arc;

use cynic::http::ReqwestExt;
use serde_json::Value;

use crate::client::system::command::queries::execute;
use crate::client::InexorRgfClient;
use crate::client::InexorRgfClientExecutionError;

#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "crate::schema::schema")]
pub mod queries {
    use crate::schema::property_instance::PropertyInstance;
    use crate::schema::scalar::Json;
    use serde_json::Value;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct ExecuteCommandVariables {
        pub name: String,
        pub args: Option<Json>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "ExecuteCommandVariables")]
    pub struct ExecuteCommand {
        pub system: MutationSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "ExecuteCommandVariables")]
    pub struct MutationSystem {
        pub commands: MutationCommands,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "ExecuteCommandVariables")]
    pub struct MutationCommands {
        #[arguments(name: $name, args: $args)]
        pub execute: Option<PropertyInstance>,
    }

    pub fn execute(name: String, args: Option<Value>) -> cynic::Operation<ExecuteCommand, ExecuteCommandVariables> {
        use cynic::MutationBuilder;
        let vars = ExecuteCommandVariables {
            name,
            args: args.map(|value| value.into()),
        };
        ExecuteCommand::build(vars)
    }
}

pub struct Command {
    client: Arc<InexorRgfClient>,
}

impl Command {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub async fn execute(&self, name: String, args: Option<Value>) -> Result<Option<Value>, InexorRgfClientExecutionError> {
        let value = self
            .client
            .client
            .post(self.client.url())
            .run_graphql(execute(name, args))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.system.commands.execute)
            .map(|property_instance| property_instance.value.0);
        Ok(value)
    }
}
