use std::process::exit;
use std::sync::Arc;

use crate::cli::system::instance::args::InstanceInfoArgs;
use crate::cli::system::instance::commands::InstanceInfoCommands;

use crate::client::InexorRgfClient;
use crate::table_model::system::instance::InstanceInfos;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn instance_info(client: &Arc<InexorRgfClient>, args: InstanceInfoArgs) {
    let Some(command) = args.commands else {
        eprintln!("[ERROR] Missing sub command");
        exit(255);
    };
    match command {
        InstanceInfoCommands::Get => match client.system().instance().get_instance_info().await {
            Ok(instance_info) => {
                println!("{}", InstanceInfos::from(vec![instance_info]).to_string());
            }
            Err(e) => eprintln!("[ERROR] Failed to get instance info:\n{e}"),
        },
    }
}
