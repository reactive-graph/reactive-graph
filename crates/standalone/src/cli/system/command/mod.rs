use std::sync::Arc;

use crate::cli::system::command::args::ExecuteCommandArgs;
use crate::client::InexorRgfClient;
use crate::client::InexorRgfClientExecutionError;

pub(crate) mod args;

pub(crate) async fn execute_command(client: &Arc<InexorRgfClient>, command_args: ExecuteCommandArgs) {
    // TODO: parse command_args
    command_args.command_arguments.spl
    match client.system().command().execute(command_args.command_name, None).await {
        Ok(Some(result)) => println!("{result}"),
        Ok(None) => println!("Command executed without return value"),
        Err(e) => match e {
            InexorRgfClientExecutionError::FailedToSendRequest(e) => {
                eprintln!("[ERROR] Failed to send request to {}\n{e:?}", client.url());
            }
            InexorRgfClientExecutionError::FailedToParseResponse(e) => {
                eprintln!("[ERROR] Failed to parse result\n{e:?}");
            }
        },
    }
}
