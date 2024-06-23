use reactive_graph_client::client::runtime::remotes::operations::add;
use std::sync::Arc;

use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NoChange;
use crate::cli::result::CommandResult;
use crate::cli::system::remotes::args::RemotesArgs;
use crate::cli::system::remotes::commands::RemotesCommands;
use crate::table_model::system::instance::InstanceInfos;
use reactive_graph_client::InexorRgfClient;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn remotes(client: &Arc<InexorRgfClient>, args: RemotesArgs) -> CommandResult {
    let Some(command) = args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        RemotesCommands::List => match client.runtime().remotes().get_all().await {
            Ok(remotes) => Ok(InstanceInfos::from(remotes).into()),
            Err(e) => Err(e.into()),
        },
        RemotesCommands::Add(address) => {
            let address = address.into();
            match client.runtime().remotes().add(&address).await {
                Ok(remote) => Ok(InstanceInfos::from(remote).into()),
                Err(e) => Err(e.into()),
            }
        }
        RemotesCommands::Remove(address) => {
            let address = address.into();
            match client.runtime().remotes().remove(&address).await {
                Ok(true) => Ok("Successfully removed remote".into()),
                Ok(false) => Err(NoChange(format!("Remote {} wasn't removed", &address.base_url()).to_string())),
                Err(e) => Err(e.into()),
            }
        }
        RemotesCommands::RemoveAll => match client.runtime().remotes().remove_all().await {
            Ok(true) => Ok("Successfully removed all remotes".into()),
            Ok(false) => Err(NoChange("No remote was removed".to_string())),
            Err(e) => Err(e.into()),
        },
        RemotesCommands::Update(address) => match client.runtime().remotes().update(&address.into()).await {
            Ok(remote) => Ok(InstanceInfos::from(remote).into()),
            Err(e) => Err(e.into()),
        },
        RemotesCommands::UpdateAll => match client.runtime().remotes().update_all().await {
            Ok(remotes) => Ok(InstanceInfos::from(remotes).into()),
            Err(e) => Err(e.into()),
        },
        RemotesCommands::FetchRemotesFromRemote(address) => match client.runtime().remotes().fetch_remotes_from_remote(&address.into()).await {
            Ok(remotes) => Ok(InstanceInfos::from(remotes).into()),
            Err(e) => Err(e.into()),
        },
        RemotesCommands::FetchRemotesFromAllRemotes => match client.runtime().remotes().fetch_remotes_from_all_remotes().await {
            Ok(remotes) => Ok(InstanceInfos::from(remotes).into()),
            Err(e) => Err(e.into()),
        },
    }
}
