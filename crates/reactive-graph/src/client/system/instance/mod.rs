use std::sync::Arc;

use crate::client::error::CommandError;
use crate::client::result::CommandResult;
use crate::client::system::instance::args::InstanceInfoArgs;
use crate::client::system::instance::commands::InstanceInfoCommands;
use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_table_model::system::instance::InstanceInfos;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn instance_info(client: &Arc<ReactiveGraphClient>, args: InstanceInfoArgs) -> CommandResult {
    let Some(command) = args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        InstanceInfoCommands::Get => match client.runtime().instance().get_instance_info().await {
            Ok(instance_info) => Ok(InstanceInfos::from(instance_info).into()),
            Err(e) => Err(e.into()),
        },
    }
}
