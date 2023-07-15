use std::alloc::System;
use std::process::exit;

use clap::Parser;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::Config;

use crate::cli::client;
use crate::cli_args::CliArguments;
use crate::cli_args::Commands;
use crate::server::server;

use inexor_rgf_client as client;
use inexor_rgf_core_config as config;
use inexor_rgf_core_model as model;
use inexor_rgf_model_runtime as model_runtime;

mod cli;
mod cli_args;
mod server;
mod table_model;

#[global_allocator]
static ALLOCATOR: System = System;

#[tokio::main]
async fn main() {
    let cli_args = CliArguments::parse();

    // Initialize logging
    if !cli_args.quiet.unwrap_or(false) {
        let logging_config_location = cli_args.logging_config.clone().unwrap_or(String::from("./config/logging.toml"));

        if let Err(error) = log4rs::init_file(&logging_config_location, Default::default()) {
            eprintln!("Failed to configure logger using config file {}: {}", &logging_config_location, error);
            let stdout = ConsoleAppender::builder().build();
            let Ok(config) = Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .build(Root::builder().appender("stdout").build(LevelFilter::Info))
            else {
                eprintln!("Failed to create fallback logger! Exiting with error");
                exit(1);
            };
            if let Err(error) = log4rs::init_config(config) {
                eprintln!("Failed to configure logger: {}", error);
            }
        }
    }
    match cli_args.commands {
        Some(commands) => match commands {
            Commands::Client(args) => client(args).await,
        },
        None => server(cli_args).await,
    }
}
