use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use reactive_graph_runtime_api::Runtime;
use reactive_graph_runtime_impl::RuntimeBuilder;

#[tokio::test(flavor = "multi_thread")]
async fn integration_test_shutdown() {
    reactive_graph_test_utils::init_logger();
    let start = Instant::now();
    RuntimeBuilder::new()
        .ignore_config_files()
        .disable_all_plugins(true)
        .pick_free_port()
        .init()
        .await
        .post_init()
        .await
        .spawn()
        .await
        .with_runtime(|runtime: Arc<dyn Runtime + Send + Sync>| async move {
            let inner_runtime = runtime.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(500)).await;
                inner_runtime.get_shutdown_manager().do_shutdown();
            });
        })
        .await
        .wait_for_stopped_with_timeout(Duration::from_secs(5))
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
    let elapsed = start.elapsed();
    // It takes up to 100ms for the GraphQL server to shutdown and after that
    // up to 100ms for the runtime to shutdown plus some millis (500+100+100+x < 800).
    assert!(elapsed > Duration::from_millis(500), "Shutdown at earliest after 500ms");
    assert!(elapsed < Duration::from_millis(1600), "Shutdown at latest after 1600ms");
}
