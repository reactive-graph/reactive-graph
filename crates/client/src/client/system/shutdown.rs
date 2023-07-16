#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "crate::schema::schema")]
pub mod mapping {
    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation")]
    pub struct Shutdown {
        pub system: ShutdownSystem,
    }

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "MutationSystem")]
    pub struct ShutdownSystem {
        pub shutdown: bool,
    }
}

pub mod mutations {
    use crate::client::system::shutdown::mapping::Shutdown;

    pub fn shutdown() -> cynic::Operation<Shutdown, ()> {
        use cynic::MutationBuilder;
        Shutdown::build(())
    }
}

pub mod api {
    use std::sync::Arc;

    use crate::client::system::shutdown::mutations::shutdown;
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
            self.client.run_graphql(shutdown(), |data| data.system.shutdown.into()).await
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;
    use std::time::Duration;

    use inexor_rgf_rt::runtime::Runtime;
    use inexor_rgf_rt::runtime::RuntimeBuilder;

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
            .with_runtime(|runtime: Arc<dyn Runtime>| async move {
                let count_before = Arc::strong_count(&runtime);

                // Check that the shutdown manager is not shutting down and the runtime is in running state
                assert!(!runtime.get_shutdown_manager().is_shutdown());
                assert!(runtime.is_running());

                let address = runtime.get_instance_service().get_instance_info().address();

                let client = InexorRgfClient::new(address).expect("Cannot create client");
                let result = client.system().shutdown().shutdown().await.expect("Failed to send shutdown command");
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
