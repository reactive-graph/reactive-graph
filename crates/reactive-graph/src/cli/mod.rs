use std::process::exit;

use colored::Colorize;

use reactive_graph_client::ReactiveGraphClient;

use crate::cli::args::ClientArgs;
use crate::cli::handler::handle_command;
use crate::cli::repl::repl;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod instances;
pub(crate) mod output_format;
pub(crate) mod repl;
pub(crate) mod result;
pub(crate) mod system;
pub(crate) mod types;

pub(crate) async fn client(client_args: ClientArgs) {
    let client = match ReactiveGraphClient::new(&client_args) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("{}: {}", "Failed to create client".red(), e);
            exit(255);
        }
    };
    // If no command was given, enter the REPL mode
    let Some(command) = client_args.commands else {
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
