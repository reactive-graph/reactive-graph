#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mapping {
    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation")]
    pub struct Shutdown {
        pub shutdown: bool,
    }
}

pub mod mutations {
    use crate::client::runtime::shutdown::mapping::Shutdown;

    pub fn shutdown() -> cynic::Operation<Shutdown, ()> {
        use cynic::MutationBuilder;
        Shutdown::build(())
    }
}

pub mod api {
    use crate::client::runtime::shutdown::mutations::shutdown;
    use std::sync::Arc;

    use crate::InexorRgfClient;
    use crate::InexorRgfClientExecutionError;

    pub struct Shutdown {
        client: Arc<InexorRgfClient>,
    }

    impl Shutdown {
        pub fn new(client: Arc<InexorRgfClient>) -> Self {
            Self { client }
        }

        pub async fn shutdown(&self) -> Result<bool, InexorRgfClientExecutionError> {
            self.client.execute_runtime(shutdown(), |data| data.shutdown).await
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;
    use std::time::Duration;

    use reactive_graph_runtime_api::Runtime;
    use reactive_graph_runtime_impl::RuntimeBuilder;

    use crate::InexorRgfClient;

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

                // Check that the shutdown manager is not shutting down and the runtime is in running state
                assert!(!runtime.get_shutdown_manager().is_shutdown());
                assert!(runtime.is_running());

                let address = runtime.get_instance_service().get_instance_info().address();

                let client = InexorRgfClient::new(address).expect("Cannot create client");
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
