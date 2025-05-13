use crate::server::graphql_schema::commands::GraphqlSchemaCommands;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(subcommand_required = true)]
pub struct GraphqlSchemaArguments {
    #[command(subcommand)]
    pub commands: GraphqlSchemaCommands,
}
