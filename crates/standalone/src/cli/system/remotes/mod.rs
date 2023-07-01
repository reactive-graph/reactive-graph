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
    }
}
