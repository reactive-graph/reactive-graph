#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-runtime-schema.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mutations {
    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation")]
    pub struct Shutdown {
        pub shutdown: bool,
    }

    pub fn shutdown() -> cynic::Operation<Shutdown, ()> {
        use cynic::MutationBuilder;
        Shutdown::build(())
    }
}

#[cfg(all(test, feature = "integration-tests"))]
pub mod test {
    use reactive_graph_runtime_api::Runtime;
    use reactive_graph_runtime_impl::RuntimeBuilder;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;

    use crate::ReactiveGraphClient;

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
                sleep(Duration::from_millis(2000)).await;

                let count_before = Arc::strong_count(&runtime);

                // Check that the shutdown manager is not shutting down and the runtime is in running state
                assert!(!runtime.get_shutdown_manager().is_shutdown());
                assert!(runtime.is_running());

                let address = runtime.get_instance_service().get_instance_info().address();

                let client = ReactiveGraphClient::new(address).expect("Cannot create client");
                let result = client.runtime().shutdown().shutdown().await.expect("Failed to send shutdown command");
                assert!(result);

                // Check that the shutdown manager is shutting down but the runtime still running
                assert!(runtime.get_shutdown_manager().is_shutdown());
                assert!(runtime.is_running());

                // Wait for runtime has stopped within 10 seconds
                assert!(runtime.wait_for_stopped_with_timeout(Duration::from_secs(10)).await.is_ok());

                // Check that the runtime is stopped
                assert!(!runtime.is_running());

                // Check that the count of atomic references has reduced externally
                assert!(count_before > Arc::strong_count(&runtime));
            })
            .await;
    }
}
