#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-runtime-schema.graphql"#, module = "crate::schema_runtime::schema")]
pub mod queries {
    use crate::InstanceInfo;

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllRemotes {
        pub remotes: Vec<InstanceInfo>,
    }

    pub fn get_all() -> cynic::Operation<GetAllRemotes, ()> {
        use cynic::QueryBuilder;
        GetAllRemotes::build(())
    }
}

#[cfg(all(test, feature = "integration-tests"))]
pub mod test {
    use crate::ReactiveGraphClient;
    use reactive_graph_runtime_api::Runtime;
    use reactive_graph_runtime_impl::RuntimeBuilder;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_all_remotes() {
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

                let remotes_manager = runtime.get_remotes_manager();

                let rt_address = runtime.address();

                // RT: Create remote
                remotes_manager.add(&rt_address).await.expect("Failed to add self to list of remotes");

                let rt_remotes = remotes_manager.get_all();

                // Client: Connect to self and get all remotes
                let client = ReactiveGraphClient::new(rt_address).expect("Cannot create client");
                let remotes = client.runtime().remotes().get_all().await.expect("Failed to get all remotes");

                // Expect that the remotes of the runtime are the same
                assert_eq!(remotes.len(), 1);
                assert_eq!(remotes, rt_remotes);
            })
            .await
            .stop()
            .await
            .pre_shutdown()
            .await
            .shutdown()
            .await;
    }
}
