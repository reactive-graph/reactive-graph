use std::sync::Arc;

use crate::client::error::CommandError;
use crate::client::result::CommandResult;
use crate::client::system::command::args::ExecuteCommandArgs;
use reactive_graph_client::ReactiveGraphClient;

pub(crate) mod args;

pub(crate) async fn execute_command(client: &Arc<ReactiveGraphClient>, command_args: ExecuteCommandArgs) -> CommandResult {
    // TODO: parse command_args
    match client.runtime().command().execute(command_args.command_name, None).await {
        Ok(Some(result)) => Ok(result.into()),
        Ok(None) => Err(CommandError::NoContent("Command executed without return value".to_string())),
        Err(e) => Err(e.into()),
    }
}
