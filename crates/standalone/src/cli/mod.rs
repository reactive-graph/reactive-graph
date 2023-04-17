use crate::cli::args::ClientArgs;
use crate::cli::commands::ClientCommands;
use std::process::exit;

use crate::cli::system::command::execute_command;
use crate::cli::system::plugin::plugins;
use crate::cli::types::components::components;
use crate::client::client::InexorRgfClient;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod system;
pub(crate) mod types;

pub(crate) async fn client(client_args: ClientArgs) {
    let client_config = (&client_args).into();
    let Ok(client) = InexorRgfClient::new_from_config(client_config) else {
        eprintln!("[ERROR] Failed to create client");
        exit(255);
    };
    let Some(command) = client_args.commands else {
        eprintln!("[ERROR] Unknown command");
        exit(255);
    };
    match command {
        ClientCommands::ExecuteCommand(args) => execute_command(&client, args).await,
        ClientCommands::Plugins(args) => plugins(&client, args).await,
        ClientCommands::Components(args) => components(&client, args).await,
        // TODO: Entity Types
        // TODO: Relation Types
        // TODO: Flow Types
        // TODO: Entity Instances
        // TODO: Relation Instances
        // TODO: Flow Instances
    }
}
