#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-runtime-schema.graphql"#, module = "crate::schema_runtime::schema")]
pub mod queries {
    use crate::schema_runtime::instance::InstanceInfo;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetInstanceInfo {
        pub instance_info: InstanceInfo,
    }

    pub fn get_instance_info() -> cynic::Operation<GetInstanceInfo, ()> {
        use cynic::QueryBuilder;
        GetInstanceInfo::build(())
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
    async fn test_get_instance_info() {
        RuntimeBuilder::new()
            .ignore_config_files()
            .disable_all_plugins(true)
            .instance_name("iojasdf")
            .instance_description("oijasdfnmei")
            .pick_free_port()
            .init()
            .await
            .post_init()
            .await
            .spawn()
            .await
            .with_runtime(|runtime: Arc<dyn Runtime + Send + Sync>| async move {
                sleep(Duration::from_millis(2000)).await;

                // Get instance info from the runtime
                let rt_instance_info = runtime.get_instance_service().get_instance_info();
                let rt_address = rt_instance_info.address();

                // Create a client
                let client = ReactiveGraphClient::new(rt_address.clone()).expect("Cannot create client");

                // Fetch instance info via client & graphql
                let instance_info = client.runtime().instance().get_instance_info().await.expect("Cannot fetch instance info");

                assert_eq!(instance_info.address.hostname, rt_address.hostname);
                assert_eq!(u16::try_from(instance_info.address.port).expect("invalid port number"), rt_address.port);
                assert_eq!(instance_info.address.secure, rt_address.secure);
                assert_eq!(instance_info.name, rt_instance_info.name);
                assert_eq!(instance_info, rt_instance_info);
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
