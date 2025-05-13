#[cfg(target_os = "linux")]
use crate::server::daemon::args::DaemonArguments;
use crate::server::graphql_schema::args::GraphqlSchemaArguments;
use crate::server::json_schema::args::JsonSchemaArguments;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ServerCommands {
    /// Runs the server as daemon.
    #[cfg(target_os = "linux")]
    Daemon(DaemonArguments),
    /// Prints the GraphQL schema and exits.
    #[clap()]
    GraphqlSchema(GraphqlSchemaArguments),
    /// Prints the JSON schema and exits.
    JsonSchema(JsonSchemaArguments),
}
