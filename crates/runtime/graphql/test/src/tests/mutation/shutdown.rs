use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;

use inexor_rgf_runtime_api::Runtime;
use inexor_rgf_runtime_impl::RuntimeBuilder;

#[derive(Debug, Deserialize)]
struct Data {
    shutdown: bool,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_shutdown() {
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
            let count_before = Arc::strong_count(&runtime);
            let query_service = runtime.get_runtime_query_service();

            // Check that the shutdown manager is not shutting down and the runtime is in running state
            assert!(!runtime.get_shutdown_manager().is_shutdown());
            assert!(runtime.is_running());

            const MUTATION_SHUTDOWN: &str = include_str!("shutdown.graphql");

            let response = query_service.query_response(MUTATION_SHUTDOWN).await;
            println!("{:?}", response.errors);
            assert!(response.errors.is_empty(), "{:?}", response.errors);
            let data = response.data.into_json().expect("Failed to get json data from graphql response");
            println!("{}", data);
            let data: Data = serde_json::from_value(data).expect("Failed to deserialize json into target data model");

            // Check that the GraphQL endpoint returned that it had accepted the shutdown command
            assert!(data.shutdown);

            // Check that the shutdown manager is shutting down but the runtime still running
            assert!(runtime.get_shutdown_manager().is_shutdown());
            assert!(runtime.is_running());

            // Wait for runtime has stopped within 5 seconds
            assert!(runtime.wait_for_stopped_with_timeout(Duration::from_secs(5)).await.is_ok());

            // Check that the runtime is stopped
            assert!(!runtime.is_running());

            // Check that the count of atomic references has reduced externally
            assert!(count_before > Arc::strong_count(&runtime));
        })
        .await;
}
