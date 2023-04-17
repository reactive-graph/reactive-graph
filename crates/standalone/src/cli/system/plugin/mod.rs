use std::process::exit;
use std::sync::Arc;
use tabled::settings::object::Rows;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;

use tabled::Table;

use crate::cli::system::plugin::args::PluginsArgs;
use crate::cli::system::plugin::commands::PluginsCommands;
use crate::client::schema::plugin::Plugin;
use crate::client::InexorRgfClient;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn plugins(client: &Arc<InexorRgfClient>, plugins_args: PluginsArgs) {
    let Some(command) = plugins_args.commands else {
        eprintln!("[ERROR] Missing sub command");
        exit(255);
    };
    match command {
        PluginsCommands::List => match client.system().plugins().get_all().await {
            Ok(Some(plugins)) => print_plugins(plugins),
            Ok(None) => println!("Plugin doesn't exist"),
            // TODO: interpret / standardize errors
            Err(e) => eprintln!("[ERROR] Failed to get all plugins:\n{e}"),
        },
        PluginsCommands::Search(args) => match client.system().plugins().search(args.into()).await {
            Ok(Some(plugins)) => print_plugins(plugins),
            Ok(None) => println!("Plugin doesn't exist"),
            Err(e) => eprintln!("[ERROR] Failed to get plugin:\n{e}"),
        },
        PluginsCommands::Get(args) => match client.system().plugins().get_by_name(args.name).await {
            Ok(Some(plugin)) => print_plugin(plugin),
            Ok(None) => println!("Plugin doesn't exist"),
            Err(e) => eprintln!("[ERROR] Failed to get plugin:\n{e}"),
        },
        PluginsCommands::Dependencies(args) => match client.system().plugins().get_dependencies(args.name).await {
            Ok(Some(plugins)) => print_plugins(plugins),
            Ok(None) => println!("Plugin doesn't exist"),
            Err(e) => eprintln!("[ERROR] Failed to get dependencies:\n{e}"),
        },
        PluginsCommands::Dependents(args) => match client.system().plugins().get_dependents(args.name).await {
            Ok(Some(plugins)) => print_plugins(plugins),
            Ok(None) => println!("Plugin doesn't exist"),
            Err(e) => eprintln!("[ERROR] Failed to get dependent plugins:\n{e}"),
        },
        PluginsCommands::Start(args) => match client.system().plugins().start(args.name).await {
            Ok(Some(plugin)) => print_plugin(plugin),
            Ok(None) => println!("Plugin doesn't exist"),
            Err(e) => eprintln!("[ERROR] Failed to start plugin:\n{e}"),
        },
        PluginsCommands::Stop(args) => match client.system().plugins().stop(args.name).await {
            Ok(Some(plugin)) => print_plugin(plugin),
            Ok(None) => println!("Plugin doesn't exist"),
            Err(e) => eprintln!("[ERROR] Failed to stop plugin:\n{e}"),
        },
        PluginsCommands::Restart(args) => match client.system().plugins().restart(args.name).await {
            Ok(Some(plugin)) => print_plugin(plugin),
            Ok(None) => println!("Plugin doesn't exist"),
            Err(e) => eprintln!("[ERROR] Failed to restart plugin:\n{e}"),
        },
    }
}

fn print_plugin(plugin: Plugin) {
    print_plugins(vec![plugin]);
}

fn print_plugins(plugins: Vec<Plugin>) {
    let table = Table::new(plugins)
        .with(Style::extended())
        .with(
            Modify::new(Rows::new(1..))
                .with(Width::increase(10).priority())
                .with(Width::truncate(40).suffix("...")),
        )
        .to_string();
    println!("{}", table);
}
