use crate::get_runtime;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::Config;
use std::time::Duration;

/// This starts the runtime in an async environment.
///
/// The runtime will be started including GraphQL server and fully
/// initialized. After 2 seconds the runtime will be stopped.
#[tokio::test(flavor = "multi_thread")]
async fn test_run() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .expect("Failed to create logger");
    if let Err(error) = log4rs::init_config(config) {
        eprintln!("Failed to configure logger: {}", error);
    }
    let rt = get_runtime();
    let runtime = rt.clone();
    tokio::spawn(async move {
        let runtime = runtime;
        runtime.init().await;
        runtime.post_init().await;
        runtime.run().await;
        runtime.pre_shutdown().await;
        runtime.shutdown().await;
    });
    tokio::time::sleep(Duration::from_secs(2)).await;
    rt.stop();
}
