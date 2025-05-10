use crate::server::schema::commands::SchemaCommands;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SchemaArguments {
    #[command(subcommand)]
    pub commands: SchemaCommands,
}
