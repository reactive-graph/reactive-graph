use std::alloc::System;
use std::process::exit;

use clap::Parser;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::Config;

#[cfg(feature = "client")]
use crate::cli::client;
#[cfg(feature = "server")]
use crate::server::cli_args::CliArguments;
#[cfg(feature = "server")]
use crate::server::cli_args::Commands;
#[cfg(feature = "server")]
use crate::server::server;

#[cfg(feature = "client")]
mod cli;
#[cfg(feature = "server")]
mod server;

#[global_allocator]
static ALLOCATOR: System = System;

#[tokio::main]
async fn main() {
    #[cfg(feature = "server")]
    {}
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
    if cli_args.markdown_help {
        clap_markdown::print_help_markdown::<CliArguments>();
        exit(0);
    }
    match cli_args.commands {
        Some(commands) => match commands {
            #[cfg(feature = "client")]
            Commands::Client(args) => client(args).await,
        },
        None => server(cli_args).await,
    }
}
