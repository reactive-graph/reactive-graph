use std::sync::Arc;

use crate::client::error::CommandError;
use crate::client::error::CommandError::NoChange;
use crate::client::error::CommandError::NotFound;
use crate::client::result::CommandResult;
use crate::client::system::plugin::args::PluginsArgs;
use crate::client::system::plugin::commands::PluginsCommands;
use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_table_model::system::plugin::PluginsTableContainer;

pub(crate) mod args;
pub(crate) mod commands;

pub(crate) async fn plugins(client: &Arc<ReactiveGraphClient>, plugins_args: PluginsArgs) -> CommandResult {
    let Some(command) = plugins_args.commands else {
        return Err(CommandError::MissingSubCommand);
    };
    match command {
        PluginsCommands::List => match client.runtime().plugins().get_all().await {
            Ok(plugins) => Ok(PluginsTableContainer::from(plugins).into()),
            Err(e) => Err(e.into()),
        },
        PluginsCommands::Search(args) => match client.runtime().plugins().search(args.into()).await {
            Ok(plugins) => Ok(PluginsTableContainer::from(plugins).into()),
            Err(e) => Err(e.into()),
        },
        PluginsCommands::Get(args) => match client.runtime().plugins().get_by_name(args.name.clone()).await {
            Ok(Some(plugin)) => Ok(PluginsTableContainer::from(plugin).into()),
            Ok(None) => Err(NotFound(format!("No plugin exists with name {}", args.name).to_string())),
            Err(e) => Err(e.into()),
        },
        PluginsCommands::Dependencies(args) => match client.runtime().plugins().get_dependencies(args.name.clone()).await {
            Ok(Some(plugins)) => Ok(PluginsTableContainer::from(plugins).into()),
            Ok(None) => Err(NotFound(format!("No plugin exists with name {}", args.name).to_string())),
            Err(e) => Err(e.into()),
        },
        PluginsCommands::Dependents(args) => match client.runtime().plugins().get_dependents(args.name.clone()).await {
            Ok(Some(plugins)) => Ok(PluginsTableContainer::from(plugins).into()),
            Ok(None) => Err(NotFound(format!("No plugin exists with name {}", args.name).to_string())),
            Err(e) => Err(e.into()),
        },
        PluginsCommands::Start(args) => match client.runtime().plugins().start(args.name).await {
            Ok(plugin) => Ok(PluginsTableContainer::from(plugin).into()),
            Err(e) => Err(e.into()),
        },
        PluginsCommands::Stop(args) => match client.runtime().plugins().stop(args.name).await {
            Ok(plugin) => Ok(PluginsTableContainer::from(plugin).into()),
            Err(e) => Err(e.into()),
        },
        PluginsCommands::Restart(args) => match client.runtime().plugins().restart(args.name).await {
            Ok(plugin) => Ok(PluginsTableContainer::from(plugin).into()),
            Err(e) => Err(e.into()),
        },
        PluginsCommands::Uninstall(args) => match client.runtime().plugins().uninstall(args.name).await {
            Ok(true) => Ok("Uninstalled plugin".into()),
            Ok(false) => Err(NoChange("Plugin wasn't uninstalled.".to_string())),
            Err(e) => Err(e.into()),
        },
    }
}

// fn print_plugin(plugin: Plugin) {
//     print_plugins(vec![plugin]);
// }
//
// fn print_plugins(plugins: Vec<Plugin>) {
//     let plugins: Vec<reactive-graph-table-model::system::plugin::Plugin> = plugins.into_iter().map(|p| p.into()).collect();
//     let table = Table::new(plugins)
//         .with(Style::extended())
//         .with(
//             Modify::new(Rows::new(1..))
//                 .with(Width::increase(10).priority())
//                 .with(Width::truncate(40).suffix("...")),
//         )
//         .to_string();
//     println!("{}", table);
// }
