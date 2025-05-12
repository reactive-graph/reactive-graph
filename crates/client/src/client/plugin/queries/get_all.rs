#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-plugin-schema.graphql"#, module = "crate::schema_plugin::schema")]
pub mod queries {
    use crate::schema_plugin::plugin::Plugin;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllPlugins {
        pub plugins: Vec<Plugin>,
    }

    pub fn get_all() -> cynic::Operation<GetAllPlugins, ()> {
        use cynic::QueryBuilder;
        GetAllPlugins::build(())
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
    async fn test_get_all_plugins() {
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

                let plugin_container_manager = runtime.get_plugin_container_manager();
                assert_eq!(plugin_container_manager.get_plugins().len(), 0);

                // Client: Connect to self and get all remotes
                let client = ReactiveGraphClient::new(runtime.address()).expect("Cannot create client");
                let plugins = client.plugins().get_all().await.expect("Failed to get list of plugins");
                assert_eq!(plugins.len(), 0);
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
