#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mutations {
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

    pub fn execute_command(name: String, args: Option<Value>) -> cynic::Operation<ExecuteCommand, ExecuteCommandVariables> {
        use cynic::MutationBuilder;
        let vars = ExecuteCommandVariables {
            name,
            args: args.map(|value| value.into()),
        };
        ExecuteCommand::build(vars)
    }
}
