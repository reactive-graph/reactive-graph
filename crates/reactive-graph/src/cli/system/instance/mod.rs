use std::sync::Arc;

use crate::cli::error::CommandError;
use crate::cli::result::CommandResult;
use crate::cli::system::instance::args::InstanceInfoArgs;
use crate::cli::system::instance::commands::InstanceInfoCommands;
use crate::client::InexorRgfClient;
use crate::table_model::system::instance::InstanceInfos;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn instance_info(client: &Arc<InexorRgfClient>, args: InstanceInfoArgs) -> CommandResult {
    let Some(command) = args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        InstanceInfoCommands::Get => match client.system().instance().get_instance_info().await {
            Ok(instance_info) => Ok(InstanceInfos::from(instance_info).into()),
            Err(e) => Err(e.into()),
        },
    }
}
