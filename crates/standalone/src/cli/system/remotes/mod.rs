use std::process::exit;
use std::sync::Arc;

use crate::cli::system::remotes::args::RemotesArgs;
use crate::cli::system::remotes::commands::RemotesCommands;

use crate::client::InexorRgfClient;
use crate::table_model::system::instance::InstanceInfos;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn remotes(client: &Arc<InexorRgfClient>, args: RemotesArgs) {
    let Some(command) = args.commands else {
        eprintln!("[ERROR] Missing sub command");
        exit(255);
    };
    match command {
        RemotesCommands::List => match client.system().remotes().get_all().await {
            Ok(remotes) => {
                println!("{}", InstanceInfos::from(remotes).to_string());
            }
            Err(e) => eprintln!("[ERROR] Failed to get instance info:\n{e}"),
        },
        RemotesCommands::Add(address) => match client.system().remotes().add(&address.into()).await {
            Ok(remote) => {
                println!("{}", InstanceInfos::from(remote).to_string());
            }
            Err(e) => eprintln!("[ERROR] Failed to add remote:\n{e}"),
        },
        RemotesCommands::Remove(address) => match client.system().remotes().remove(&address.into()).await {
            Ok(true) => {
                println!("Successfully removed remote");
            }
            Ok(false) => {
                println!("Remote wasn't removed");
            }
            Err(e) => eprintln!("[ERROR] Failed to remove remote:\n{e}"),
        },
        RemotesCommands::RemoveAll => match client.system().remotes().remove_all().await {
            Ok(true) => {
                println!("Successfully removed all remotes");
            }
            Ok(false) => {
                println!("No remote was removed");
            }
            Err(e) => eprintln!("[ERROR] Failed to fetch remotes from all remotes:\n{e}"),
        },
        RemotesCommands::Update(address) => match client.system().remotes().update(&address.into()).await {
            Ok(remote) => {
                println!("{}", InstanceInfos::from(remote).to_string());
            }
            Err(e) => eprintln!("[ERROR] Failed to update remote:\n{e}"),
        },
        RemotesCommands::UpdateAll => match client.system().remotes().update_all().await {
            Ok(remotes) => {
                println!("{}", InstanceInfos::from(remotes).to_string());
            }
            Err(e) => eprintln!("[ERROR] Failed to fetch remotes from all remotes:\n{e}"),
        },
        RemotesCommands::FetchRemotesFromRemote(address) => match client.system().remotes().fetch_remotes_from_remote(&address.into()).await {
            Ok(remotes) => {
                println!("{}", InstanceInfos::from(remotes).to_string());
            }
            Err(e) => eprintln!("[ERROR] Failed to fetch remotes from remote:\n{e}"),
        },
        RemotesCommands::FetchRemotesFromAllRemotes => match client.system().remotes().fetch_remotes_from_all_remotes().await {
            Ok(remotes) => {
                println!("{}", InstanceInfos::from(remotes).to_string());
            }
            Err(e) => eprintln!("[ERROR] Failed to fetch remotes from all remotes:\n{e}"),
        },
    }
}
