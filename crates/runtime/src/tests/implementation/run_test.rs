use crate::get_runtime;
use std::env;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_run() {
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
