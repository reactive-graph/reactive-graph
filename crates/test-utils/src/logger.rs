use log::LevelFilter;
use log::trace;
use log4rs::Config;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::init_config;

pub fn init_logger() {
    let encoder = PatternEncoder::new("[{f}:{L}] {h([{l}])} {m}{n}");
    let stdout = ConsoleAppender::builder().encoder(Box::new(encoder)).build();
    match Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
    {
        Ok(config) => match init_config(config) {
            Ok(_) => trace!("Test logger enabled"),
            Err(e) => eprintln!("Failed to initialize logger with config: {e}"),
        },
        Err(config_errors) => {
            for config_error in config_errors.errors() {
                eprintln!("{config_error}");
            }
        }
    }
}
