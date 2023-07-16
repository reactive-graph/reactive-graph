use std::process::exit;

use crate::cli::args::ClientArgs;
use crate::cli::commands::ClientCommands;
use crate::cli::system::command::execute_command;
use crate::cli::system::instance::instance_info;
use crate::cli::system::plugin::plugins;
use crate::cli::system::remotes::remotes;
use crate::cli::system::shutdown::shutdown;
use crate::cli::types::components::components;
use crate::client::client::InexorRgfClient;

pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod system;
pub(crate) mod types;

pub(crate) async fn client(client_args: ClientArgs) {
    let client = match InexorRgfClient::new(&client_args) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("[ERROR] Failed to create client: {}", e);
            exit(255);
        }
    };
    let Some(command) = client_args.commands else {
        eprintln!("[ERROR] Unknown command");
        exit(255);
    };
    match command {
        // System
        ClientCommands::ExecuteCommand(args) => execute_command(&client, args).await,
        ClientCommands::InstanceInfo(args) => instance_info(&client, args).await,
        ClientCommands::Plugins(args) => plugins(&client, args).await,
        ClientCommands::Remotes(args) => remotes(&client, args).await,
        ClientCommands::Shutdown => shutdown(&client).await,
        // Types
        ClientCommands::Components(args) => components(&client, args).await,
        // TODO: Entity Types
        // TODO: Relation Types
        // TODO: Flow Types
        // TODO: Entity Instances
        // TODO: Relation Instances
        // TODO: Flow Instances
    }
}
