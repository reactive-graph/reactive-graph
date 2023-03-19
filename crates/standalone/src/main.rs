use std::alloc::System;
use std::process::exit;

use clap::Parser;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::Config;
use tokio::time::Duration;

use inexor_rgf_rt::runtime::RuntimeBuilder;

use crate::cli_args::CliArguments;

mod cli_args;

#[global_allocator]
static ALLOCATOR: System = System;

#[tokio::main]
async fn main() {
    let cli_args = CliArguments::parse();

    if !cli_args.quiet.unwrap_or(false) {
        let logging_config_location = cli_args.logging_config.unwrap_or(String::from("./config/logging.toml"));

        if let Err(error) = log4rs::init_file(&logging_config_location, Default::default()) {
            eprintln!("Failed to configure logger using config file {}: {}", &logging_config_location, error);
            let stdout = ConsoleAppender::builder().build();
            let Ok(config) = Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .build(Root::builder().appender("stdout").build(LevelFilter::Info)) else {
                eprintln!("Failed to create fallback logger! Exiting with error");
                exit(1);
            };
            if let Err(error) = log4rs::init_config(config) {
                eprintln!("Failed to configure logger: {}", error);
            }
        }
    }

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
        .disable_hot_deploy(cli_args.disable_hot_deploy)
        .init()
        .await
        .post_init()
        .await
        .run()
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
