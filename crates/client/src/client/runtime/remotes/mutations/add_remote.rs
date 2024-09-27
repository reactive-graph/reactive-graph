#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mutations {
    use crate::client::runtime::instance::variables::address::variables::InstanceAddressVariables;
    use crate::client::runtime::instance::variables::address::variables::InstanceAddressVariablesFields;
    use crate::schema_runtime::InstanceInfo;

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation", variables = "InstanceAddressVariables")]
    pub struct AddRemote {
        pub remotes: AddRemoteMutationRemotes,
    }

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "MutationRemotes", variables = "InstanceAddressVariables")]
    pub struct AddRemoteMutationRemotes {
        #[arguments(address: $address)]
        pub add: InstanceInfo,
    }

    pub fn add(vars: InstanceAddressVariables) -> cynic::Operation<AddRemote, InstanceAddressVariables> {
        use cynic::MutationBuilder;
        AddRemote::build(vars)
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
    async fn test_add_remote() {
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

                // Get instance info from the runtime
                let rt_address = runtime.address();

                // Check that there are no remotes
                assert_eq!(remotes_manager.get_all().len(), 0);

                // Client: Connect to self and get all remotes
                let client = ReactiveGraphClient::new(rt_address.clone()).expect("Cannot create client");
                let remote = client.runtime().remotes().add(&rt_address).await.expect("Failed to add remote");

                // Check that there is a new remote
                assert_eq!(remotes_manager.get_all().len(), 1);
                assert_eq!(rt_address, remote.address);
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
