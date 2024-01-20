#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod queries {
    use serde_json::Value;

    // use crate::schema_runtime::property_instance::PropertyInstance;
    use crate::schema_runtime::property_instance::CommandResult;
    use crate::schema_runtime::scalar::Json;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct ExecuteCommandVariables {
        pub name: String,
        pub args: Option<Json>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "ExecuteCommandVariables")]
    pub struct ExecuteCommand {
        pub commands: MutationCommands,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "ExecuteCommandVariables")]
    pub struct MutationCommands {
        #[arguments(name: $name, args: $args)]
        pub execute: Option<CommandResult>,
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

pub mod api {
    use std::sync::Arc;

    use cynic::http::ReqwestExt;
    use serde_json::Value;

    use crate::client::runtime::command::queries::execute;
    use crate::InexorRgfClient;
    use crate::InexorRgfClientExecutionError;

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
                .post(self.client.url_runtime())
                .run_graphql(execute(name, args))
                .await
                .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
                .data
                .and_then(|data| data.commands.execute)
                .map(|property_instance| property_instance.value.0);
            Ok(value)
        }
    }
}
