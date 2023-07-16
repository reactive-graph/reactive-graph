use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;

use crate::Runtime;
use crate::RuntimeBuilder;

#[derive(Debug, Deserialize)]
struct System {
    shutdown: bool,
}

#[derive(Debug, Deserialize)]
struct Data {
    system: System,
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
        .with_runtime(|runtime: Arc<dyn Runtime>| async move {
            let count_before = Arc::strong_count(&runtime);
            let query_service = runtime.get_graphql_query_service();

            // Check that the shutdown manager is not shutting down and the runtime is in running state
            assert!(!runtime.get_shutdown_manager().is_shutdown());
            assert!(runtime.is_running());

            const MUTATION_SHUTDOWN: &str = include_str!("../../../../graphql/system/shutdown/shutdown.graphql");

            let response = query_service.query_response(MUTATION_SHUTDOWN).await;
            assert!(response.errors.is_empty());
            let data = response.data.into_json().expect("Failed to get json data from graphql response");
            println!("{}", data);
            let data: Data = serde_json::from_value(data).expect("Failed to deserialize json into target data model");

            // Check that the GraphQL endpoint returned that it had accepted the shutdown command
            assert!(data.system.shutdown);

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
