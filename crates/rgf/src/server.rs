use std::time::Duration;

use inexor_rgf_runtime_impl::RuntimeBuilder;

use crate::cli_args::CliArguments;

pub(crate) async fn server(cli_args: CliArguments) {
    RuntimeBuilder::new()
        // Locations of the config files
        .instance_config(cli_args.instance_config)
        .graphql_server_config(cli_args.graphql_config)
        .plugins_config(cli_args.plugins_config)
        .load_config_files()
        .await
        // Configure CLI arguments
        .instance_name(cli_args.instance_name)
        .instance_description(cli_args.instance_description)
        .hostname(cli_args.hostname)
        .port(cli_args.port)
        .secure(cli_args.secure)
        .shutdown_timeout(cli_args.shutdown_timeout)
        .workers(cli_args.workers)
        .default_context_path(cli_args.default_context_path)
        .disable_all_plugins(cli_args.disable_all_plugins)
        .disabled_plugins(cli_args.disabled_plugins)
        .enabled_plugins(cli_args.enabled_plugins)
        .disable_hot_deploy(cli_args.disable_hot_deploy)
        .hot_deploy_location(cli_args.hot_deploy_location)
        .install_location(cli_args.install_location)
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
        .wait_for(if cli_args.stop_immediately.unwrap_or(false) {
            Duration::from_millis(10)
        } else {
            Duration::from_secs(2)
        })
        .await;
}
