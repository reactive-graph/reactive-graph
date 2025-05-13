use crate::server::json_schema::commands::InstancesCommands;
use crate::server::json_schema::commands::JsonSchemaCommands;
use crate::server::json_schema::commands::TypesCommands;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(subcommand_required = true)]
pub struct JsonSchemaArguments {
    #[command(subcommand)]
    pub commands: JsonSchemaCommands,
}

#[derive(Parser, Debug)]
pub struct TypesArguments {
    #[command(subcommand)]
    pub commands: TypesCommands,
}

#[derive(Parser, Debug)]
pub struct InstancesArguments {
    #[command(subcommand)]
    pub commands: InstancesCommands,
}
