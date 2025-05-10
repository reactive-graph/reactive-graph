pub mod args;
pub mod commands;
#[cfg(target_os = "linux")]
pub mod daemon;

pub mod schema;

use std::time::Duration;

use reactive_graph_runtime_impl::RuntimeBuilder;

use crate::server::args::logging::init_logging;
use crate::server::schema::print_graphql_schema_and_exit;
use args::ServerArguments;

#[tokio::main]
pub async fn server(args: ServerArguments) {
    if let Some(commands) = &args.commands {
        #[allow(unreachable_patterns, clippy::collapsible_match)]
        match commands {
            #[cfg(target_os = "linux")]
            commands::ServerCommands::Daemon(_) => {
                // already handled.
            }
            commands::ServerCommands::Schema(args) => {
                print_graphql_schema_and_exit(&args.commands).await;
            }
            _ => {}
        }
    }
    init_logging(&args);
    run(args).await
}

pub async fn run(args: ServerArguments) {
    RuntimeBuilder::new()
        // Locations of the config files
        .instance_config(args.runtime.config_locations.instance_config)
        .graphql_server_config(args.runtime.config_locations.graphql_config)
        .plugins_config(args.runtime.config_locations.plugins_config)
        .load_config_files()
        .await
        // Configure CLI arguments
        .instance_name(args.runtime.instance.name)
        .instance_description(args.runtime.instance.description)
        .hostname(args.runtime.graphql_server.hostname)
        .port(args.runtime.graphql_server.port)
        .secure(args.runtime.graphql_server.secure)
        .ssl_certificate_path(args.runtime.graphql_server.ssl_certificate_path)
        .ssl_private_key_path(args.runtime.graphql_server.ssl_private_key_path)
        .shutdown_timeout(args.runtime.graphql_server.shutdown_timeout)
        .workers(args.runtime.graphql_server.workers)
        .default_context_path(args.runtime.graphql_server.default_context_path)
        .disable_all_plugins(args.runtime.plugins.disable_all_plugins)
        .disabled_plugins(args.runtime.plugins.disabled_plugins)
        .enabled_plugins(args.runtime.plugins.enabled_plugins)
        .disable_hot_deploy(args.runtime.plugins.disable_hot_deploy)
        .hot_deploy_location(args.runtime.plugins.hot_deploy_location)
        .install_location(args.runtime.plugins.install_location)
        .init()
        .await
        .post_init()
        .await
        .spawn()
        .await
        .wait_for_stopped()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await
        // Wait for 2 more seconds before exiting
        .wait_for(if args.runtime.stop_immediately.unwrap_or(false) {
            Duration::from_millis(10)
        } else {
            Duration::from_secs(2)
        })
        .await;
}
