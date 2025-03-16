use crate::server::args::ServerArguments;
use log::LevelFilter;
use log4rs::Config;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use std::process::exit;

pub fn init_logging(args: &ServerArguments) {
    // Initialize logging
    if !args.quiet.unwrap_or(false) {
        let logging_config_location = args
            .runtime
            .config_locations
            .logging_config
            .clone()
            .unwrap_or(String::from("./config/logging.toml"));

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
}
