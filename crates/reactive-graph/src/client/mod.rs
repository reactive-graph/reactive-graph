use std::process::exit;

use colored::Colorize;

use reactive_graph_client::ReactiveGraphClient;

use crate::client::args::ClientArguments;
use crate::client::handler::handle_command;
use crate::client::repl::repl;

pub mod args;
pub mod commands;
pub mod error;
pub mod handler;
pub mod instances;
pub mod introspection;
pub mod output_format;
pub mod repl;
pub mod result;
pub mod system;
pub mod types;

#[tokio::main]
pub async fn client(args: ClientArguments) {
    let client = match ReactiveGraphClient::new(&args.connection) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("{}: {}", "Failed to create client".red(), e);
            exit(255);
        }
    };
    // If no command was given, enter the REPL mode
    let Some(command) = args.commands else {
        match repl(&client).await {
            Ok(_) => exit(0),
            Err(exit_code) => exit(exit_code),
        }
    };
    // If a command was given, handle command
    match handle_command(&client, command).await {
        Ok(response) => {
            println!("{}", response);
            exit(0)
        }
        Err(e) => {
            eprintln!("{}: {}", "Command failed with error".red(), e);
            exit(e.exit_code())
        }
    }
}
