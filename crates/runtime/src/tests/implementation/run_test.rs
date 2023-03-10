use crate::get_runtime;
use std::env;
use std::time::Duration;

/// This starts the runtime in an async environment.
///
/// The runtime will be started including GraphQL server and fully
/// initialized. After 5 seconds the runtime will be stopped.
#[tokio::test(flavor = "multi_thread")]
async fn test_run() {
    // TODO: remove set_current_dir and call get_runtime with a config location
    env::set_current_dir("../..").expect("Cant change directory to repository root dir");
    if let Err(error) = log4rs::init_file("./config/logging.toml", Default::default()) {
        eprintln!("Failed to configure logger: {}", error);
    }
    let rt = get_runtime();
    let runtime = rt.clone();
    tokio::spawn(async move {
        let runtime = runtime;
        runtime.init();
        runtime.post_init();
        runtime.run().await;
        runtime.pre_shutdown();
        runtime.shutdown();
    });
    tokio::time::sleep(Duration::from_secs(5)).await;
    rt.stop();
}
